#!/usr/bin/env python3
"""Run Tidal conformance tests across the compiler matrix.

Each test is a .cpp file under tests/ with directive comments at the top
(see documentation/specification-format.md). Compilers come from tools/compilers.json (see
tools/matrix.py). Each test is compiled with every selected compiler, run
if it expects `run` and the compiler can build executables, and compared
with its expectation.

Only `baseline` compilers decide whether a test passes. Results from
`report` compilers are shown but don't fail the run.

A run that exits 0 still fails when Clang's type sanitizer
(`-fsanitize=type`) printed a report, because that sanitizer cannot stop
the program.

A `tidal-interpreter: run-fail` line is for the front end's interpreter
only (decision 0053). The runner checks its form and does not use it, so
the matrix compilers only build such a test.

Results are cached in build/cache/, keyed by the test text, the compiler's
identity (version output and image ID), and its command templates.

Usage:
    tools/run_tests.py [paths...] [--compilers baseline|all|ID,ID]
                       [--report tests/RESULTS.md] [--no-cache] [-j N]
"""

import argparse
import concurrent.futures
import fnmatch
import hashlib
import json
import os
import re
import subprocess
import sys
from dataclasses import dataclass, field
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import matrix  # noqa: E402
from matrix import BUILD, TESTS, Compiler  # noqa: E402

EXPECTATIONS = {"accept", "reject", "link-fail", "run", "run-fail"}
# The values of tidal-interpreter, which only the front end's interpreter
# checks (decision 0053).
INTERPRETER_EXPECTATIONS = {"run-fail"}
# Bump when outcome() changes how it computes a result, so cached results
# from the old logic are not reused.
OUTCOME_VERSION = 5
EXTRA_SUFFIX = ".tu.cpp"  # extra translation units, not tests themselves
DIRECTIVE = re.compile(r"^//\s*tidal-([a-z0-9.-]+):\s*(.*?)\s*$")
CACHE = BUILD / "cache"


@dataclass(eq=False)
class Test:
    path: Path
    rules: list[str]
    default: str | None
    overrides: dict[str, str]
    flags: list[str] = field(default_factory=list)
    notes: list[str] = field(default_factory=list)
    sources: list[Path] = field(default_factory=list)
    errors: dict[str, str] = field(default_factory=dict)
    # The tidal-interpreter expectation, or None. No compiler uses it.
    interpreter: str | None = None
    # The tidal-allows-edg rule, or None. No compiler uses it; the front
    # end's harness judges EDG's answer by this rule alone.
    allows_edg: str | None = None

    @property
    def name(self) -> str:
        return str(self.path.relative_to(TESTS))

    def expect(self, c: Compiler) -> str | None:
        """Expected outcome for one compiler: by id, the id it is a variant of, family, default.

        None means the test states no expectation for this compiler. That is
        an error for a baseline compiler and informational for a report one.
        """
        e = (self.overrides.get(c.id) or self.overrides.get(c.variant_of)
             or self.overrides.get(c.family) or self.default)
        if e is None and c.role == "baseline":
            raise ValueError(f"{self.name}: no expectation for baseline compiler {c.id}")
        if e in {"run", "run-fail", "link-fail"} and c.frontend_only:
            return "accept"
        return e

    def error_pattern(self, c: Compiler) -> str | None:
        """The pattern a rejection by this compiler must match: by id, the id it is a variant of, family, default."""
        return (self.errors.get(c.id) or self.errors.get(c.variant_of)
                or self.errors.get(c.family) or self.errors.get(""))


def parse_test(path: Path) -> Test:
    rules, flags, notes, sources = [], [], [], []
    default = None
    overrides = {}
    errors = {}
    interpreter = None
    allows_edg = None
    for line in path.read_text().splitlines():
        if not line.startswith("//"):
            break
        m = DIRECTIVE.match(line)
        if not m:
            continue
        key, value = m.groups()
        if key == "rules":
            rules += value.split()
        elif key == "expect":
            default = value
        elif key.startswith("expect-"):
            overrides[key.removeprefix("expect-")] = value
        elif key == "error" or key.startswith("error-"):
            target = key.removeprefix("error").removeprefix("-")
            try:
                re.compile(value)
            except re.error as e:
                raise ValueError(f"{path}: bad tidal-{key} pattern: {e}")
            errors[target] = value
        elif key == "flags":
            flags += value.split()
        elif key == "note":
            notes.append(value)
        elif key == "sources":
            sources += [path.parent / s for s in value.split()]
        elif key == "interpreter":
            interpreter = value
        elif key == "allows-edg":
            allows_edg = value
        else:
            raise ValueError(f"{path}: unknown directive tidal-{key}")
    if not rules:
        raise ValueError(f"{path}: missing tidal-rules")
    if default is None and not overrides:
        raise ValueError(f"{path}: missing tidal-expect")
    for value in [default, *overrides.values()]:
        if value is not None and value not in EXPECTATIONS:
            raise ValueError(f"{path}: bad expectation {value!r}")
    if interpreter is not None:
        if interpreter not in INTERPRETER_EXPECTATIONS:
            raise ValueError(f"{path}: bad tidal-interpreter value {interpreter!r}")
        if {default, *overrides.values()} - {None, "accept"}:
            raise ValueError(f"{path}: tidal-interpreter needs a test that expects accept on every compiler")
    for s in sources:
        if not s.name.endswith(EXTRA_SUFFIX) or not s.exists():
            raise ValueError(f"{path}: tidal-sources entry {s.name} must be an existing *{EXTRA_SUFFIX} file")
    return Test(path, rules, default, overrides, flags, notes, sources, errors, interpreter, allows_edg)


def test_files(paths: list[Path]) -> list[Path]:
    """Test files under the given paths, leaving out extra translation units."""
    files = []
    for p in paths:
        p = p.resolve()
        files += sorted(p.rglob("*.cpp")) if p.is_dir() else [p]
    return [f for f in files if not f.name.endswith(EXTRA_SUFFIX)]


LINK_DIAG = re.compile(r"(multiple definition of|undefined reference to) \S+")
DIAG = re.compile(r'^(?:\S+:\d+:\d+|"[^"]*"\s*, line \d+): (error|note): (.*)$')


def diagnostics(stderr: str) -> list[str]:
    """Return "error: ..." and "note: ..." messages, without locations.

    Handles GCC/Clang ("file:1:2: error: msg") and EDG ('"file", line 1:
    error: msg', with long lines wrapped onto lines indented by 10 spaces).
    """
    text = re.sub(r"\n {10}(?=\S)", " ", stderr)
    found = []
    for line in text.splitlines():
        if m := LINK_DIAG.search(line):
            found.append(f"link: {m.group(0)}")
        elif m := DIAG.match(line):
            msg = f"{m.group(1)}: {m.group(2)}"
            if "in call to" not in msg and "declared here" not in msg:
                found.append(msg)
    return found


def failure(proc: subprocess.CompletedProcess) -> str:
    """All error and note messages of a failed compile or link, joined with " / ".

    The whole list is kept, so a tidal-error pattern can match any of them.
    Notes count: Clang and EDG name the cause of a failed constant
    evaluation only in a note.
    """
    diags = diagnostics(proc.stderr) or [" ".join(proc.stderr.split())[:200]]
    return " / ".join(diags)


def short(detail: str) -> str:
    errors = [d for d in detail.split(" / ") if not d.startswith("note:")] or detail.split(" / ")
    return " / ".join(errors[:2])


# A report from a sanitizer that cannot stop the program. Clang 22's type
# sanitizer prints its report and lets the program go on, and no option
# makes it exit nonzero. Its report counts as a run failure.
CONTINUING_REPORT = re.compile(r"^==\d+==ERROR: TypeSanitizer:", re.M)


def run_binary(c: Compiler, out: str) -> tuple[str, str]:
    try:
        run = c.run([out], timeout=60)
    except subprocess.TimeoutExpired:
        return "run-fail", "timeout"
    if run.returncode != 0 or CONTINUING_REPORT.search(run.stdout + run.stderr):
        # Keep every line, so a tidal-error pattern can match a sanitizer's report.
        lines = [" ".join(l.split()) for l in (run.stdout + run.stderr).splitlines() if l.strip()]
        return "run-fail", " / ".join([f"exit {run.returncode}", *lines])[:4000]
    return "run", ""


def multi_unit_outcome(test: Test, c: Compiler) -> tuple[str, str]:
    """Compile each translation unit on its own, then link them together."""
    srcs, out = [], None
    for i, path in enumerate([test.path, *test.sources]):
        src, unit_out, _ = c.paths(path)
        if out is None:
            out = unit_out
        srcs.append(src)
        proc = c.run(c.argv(c.check, src, f"{out}-tu{i}", test.flags))
        if proc.returncode != 0:
            return "reject", failure(proc)
    if c.frontend_only:
        return "accept", ""
    proc = c.run(c.argv(c.build, srcs, out, test.flags))
    if proc.returncode != 0:
        return "link-fail", failure(proc)
    if test.expect(c) in {"run", "run-fail"}:
        return run_binary(c, out)
    return "accept", ""


def outcome(test: Test, c: Compiler) -> tuple[str, str]:
    """Return (outcome, detail): accept, reject, link-fail, run, run-fail, or skip."""
    if any(fnmatch.fnmatchcase(f, u) for f in test.flags for u in c.unsupported_flags):
        return "skip", "unsupported flags"
    if not all(f in test.flags for f in c.required_flags):
        return "skip", "needs flags " + " ".join(c.required_flags)
    if test.sources:
        return multi_unit_outcome(test, c)
    src, out, _ = c.paths(test.path)
    if test.expect(c) == "link-fail":
        # Check first, so a compile error is not reported as a link failure.
        proc = c.run(c.argv(c.check, src, out, test.flags))
        if proc.returncode != 0:
            return "reject", failure(proc)
        proc = c.run(c.argv(c.build, src, out, test.flags))
        if proc.returncode != 0:
            return "link-fail", failure(proc)
        return "accept", ""
    wants_run = test.expect(c) in {"run", "run-fail"}
    template = c.build if wants_run else c.check
    proc = c.run(c.argv(template, src, out, test.flags))
    if proc.returncode != 0:
        return "reject", failure(proc)
    if not wants_run:
        return "accept", ""
    return run_binary(c, out)


INCLUDE = re.compile(r'^\s*#\s*include\s*"([^"]+)"', flags=re.M)


def included(sources: list[Path]) -> list[str]:
    """The text of every file that the sources #include "...", transitively, in a fixed order."""
    seen: dict[Path, str] = {}
    todo = list(sources)
    while todo:
        src = todo.pop()
        for name in INCLUDE.findall(src.read_text()):
            f = (src.parent / name).resolve()
            if f.is_file() and f not in seen:
                seen[f] = f.read_text()
                todo.append(f)
    return [seen[f] for f in sorted(seen)]


def cached_outcome(test: Test, c: Compiler, identity: str, use_cache: bool) -> tuple[str, str]:
    headers = included([test.path, *test.sources])
    key = hashlib.sha256(json.dumps([
        [s.read_text() for s in [test.path, *test.sources]],
        c.id, identity, c.check, c.build, sorted(c.unsupported_flags), test.expect(c), OUTCOME_VERSION,
        # Only compilers that use these fields add them, so other cache keys stay as they were.
        *([sorted(c.required_flags), sorted(c.drop_flags)] if c.required_flags or c.drop_flags else []),
        # Only tests that include files add their text.
        *([headers] if headers else []),
    ]).encode()).hexdigest()
    entry = CACHE / f"{key}.json"
    if use_cache and entry.exists():
        entry.touch()  # marks the entry as used, so CI can prune the others
        return tuple(json.loads(entry.read_text()))
    result = outcome(test, c)
    if result == ("run-fail", "timeout"):
        return result  # transient under load; don't cache
    CACHE.mkdir(parents=True, exist_ok=True)
    entry.write_text(json.dumps(result))
    return result


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("paths", nargs="*", type=Path, default=[TESTS])
    ap.add_argument("--compilers", default="baseline")
    ap.add_argument("--report", type=Path, help="write a Markdown results table")
    ap.add_argument("--no-cache", action="store_true")
    ap.add_argument("--errors-out", type=Path,
                    help="write the error messages of every rejecting compiler as JSON")
    ap.add_argument("-j", type=int, default=os.cpu_count())
    args = ap.parse_args()

    tests = [parse_test(f) for f in test_files(args.paths)]
    compilers = matrix.select(args.compilers)

    try:
        for c in compilers:
            c.start()
        identities = {c: c.identity() for c in compilers}
        versions = {c: c.version_line() for c in compilers}
        jobs = [(t, c) for t in tests for c in compilers]
        with concurrent.futures.ThreadPoolExecutor(args.j) as pool:
            results = dict(zip(jobs, pool.map(
                lambda job: cached_outcome(*job, identities[job[1]], not args.no_cache), jobs)))
    finally:
        for c in compilers:
            c.stop()

    def matches(t: Test, c: Compiler) -> bool:
        got, detail = results[(t, c)]
        if got == "skip" or t.expect(c) is None:
            return True
        if got != t.expect(c):
            return False
        pattern = t.error_pattern(c) if got in {"reject", "link-fail", "run-fail"} else None
        return pattern is None or re.search(pattern, detail, re.IGNORECASE) is not None

    failures, rows, differs = 0, [], []
    for t in tests:
        bad = [c for c in compilers if not matches(t, c)]
        failed = any(c.role == "baseline" for c in bad)
        failures += failed
        rows.append((t, "FAIL" if failed else "pass"))
        for c in bad:
            got, detail = results[(t, c)]
            if got == t.expect(c):
                line = (f"{t.name} [{c.id}]: {got}, but no error matches "
                        f"/{t.error_pattern(c)}/  {short(detail)}")
            else:
                line = f"{t.name} [{c.id}]: expected {t.expect(c)}, got {got}  {short(detail)}"
            if c.role == "baseline":
                print("FAIL " + line)
            else:
                differs.append(line)

    for line in differs:
        print("differs " + line)

    if args.errors_out:
        dump = {}
        for t in tests:
            for c in compilers:
                got, detail = results[(t, c)]
                if got in {"reject", "link-fail", "run-fail"}:
                    dump.setdefault(t.name, {})[c.id] = detail.split(" / ")
        args.errors_out.write_text(json.dumps(dump, indent=1, sort_keys=True))
    print(f"{len(tests) - failures}/{len(tests)} tests pass on baseline compilers; "
          f"{len(differs)} differences on report compilers")

    if args.report:
        ids = [c.id for c in compilers]
        lines = [
            "# Test results",
            "",
            "Generated by `tools/run_tests.py --report`. Do not edit by hand.",
            "A cell marked ✗ differs from the test's expectation. Only `baseline`",
            "compilers affect the Status column.",
            "",
            *(f"- `{c.id}` ({c.role}): `{versions[c]}`" for c in compilers),
            "",
            "| Test | Rules | Expected | " + " | ".join(ids) + " | Status |",
            "|---|---|---|" + "---|" * len(ids) + "---|",
        ]
        for t, status in rows:
            cells = " | ".join(
                results[(t, c)][0] + ("" if matches(t, c) else " ✗") for c in compilers
            )
            expected = t.default or ", ".join(f"{k}: {v}" for k, v in t.overrides.items())
            lines.append(f"| `{t.name}` | {' '.join(t.rules)} | {expected} | {cells} | {status} |")
        lines += [
            "",
            f"{len(tests) - failures}/{len(tests)} tests pass on baseline compilers.",
            "",
            "## Rejections",
            "",
            "First error from each compiler that rejected a test, so a reviewer can",
            "check that the rejection has the intended cause.",
            "",
        ]
        for t, _ in rows:
            for c in compilers:
                if results[(t, c)][0] in {"reject", "link-fail"}:
                    detail = results[(t, c)][1].replace("|", "\\|")
                    lines.append(f"- `{t.name}` ({c.id}): {detail}")
        lines.append("")
        args.report.write_text("\n".join(lines))

    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
