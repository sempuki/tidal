#!/usr/bin/env python3
"""Run Tidal conformance tests across the compiler matrix.

Each test is a .cpp file under tests/ with directive comments at the top
(see docs/spec-format.md). Compilers come from tools/compilers.json (see
tools/matrix.py). Each test is compiled with every selected compiler, run
if it expects `run` and the compiler can build executables, and compared
with its expectation.

Only `baseline` compilers decide whether a test passes. Results from
`report` compilers are shown but don't fail the run.

Results are cached in build/cache/, keyed by the test text, the compiler's
identity (version output and image ID), and its command templates.

Usage:
    tools/run_tests.py [paths...] [--compilers baseline|all|ID,ID]
                       [--report tests/RESULTS.md] [--no-cache] [-j N]
"""

import argparse
import concurrent.futures
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
# Bump when outcome() changes how it computes a result, so cached results
# from the old logic are not reused.
OUTCOME_VERSION = 2
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

    @property
    def name(self) -> str:
        return str(self.path.relative_to(TESTS))

    def expect(self, c: Compiler) -> str | None:
        """Expected outcome for one compiler: by id, then family, then default.

        None means the test states no expectation for this compiler. That is
        an error for a baseline compiler and informational for a report one.
        """
        e = self.overrides.get(c.id) or self.overrides.get(c.family) or self.default
        if e is None and c.role == "baseline":
            raise ValueError(f"{self.name}: no expectation for baseline compiler {c.id}")
        if e in {"run", "run-fail", "link-fail"} and c.frontend_only:
            return "accept"
        return e


def parse_test(path: Path) -> Test:
    rules, flags, notes, sources = [], [], [], []
    default = None
    overrides = {}
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
        elif key == "flags":
            flags += value.split()
        elif key == "note":
            notes.append(value)
        elif key == "sources":
            sources += [path.parent / s for s in value.split()]
        else:
            raise ValueError(f"{path}: unknown directive tidal-{key}")
    if not rules:
        raise ValueError(f"{path}: missing tidal-rules")
    if default is None and not overrides:
        raise ValueError(f"{path}: missing tidal-expect")
    for value in [default, *overrides.values()]:
        if value is not None and value not in EXPECTATIONS:
            raise ValueError(f"{path}: bad expectation {value!r}")
    for s in sources:
        if not s.name.endswith(EXTRA_SUFFIX) or not s.exists():
            raise ValueError(f"{path}: tidal-sources entry {s.name} must be an existing *{EXTRA_SUFFIX} file")
    return Test(path, rules, default, overrides, flags, notes, sources)


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
    diags = diagnostics(proc.stderr) or [" ".join(proc.stderr.split())[:200]]
    return " / ".join(diags[:2])


def run_binary(c: Compiler, out: str) -> tuple[str, str]:
    try:
        run = c.run([out], timeout=60)
    except subprocess.TimeoutExpired:
        return "run-fail", "timeout"
    if run.returncode != 0:
        return "run-fail", f"exit {run.returncode}: {(run.stdout + run.stderr).strip()[:200]}"
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
    if any(f in c.unsupported_flags for f in test.flags):
        return "skip", "unsupported flags"
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


def cached_outcome(test: Test, c: Compiler, identity: str, use_cache: bool) -> tuple[str, str]:
    key = hashlib.sha256(json.dumps([
        [s.read_text() for s in [test.path, *test.sources]],
        c.id, identity, c.check, c.build, sorted(c.unsupported_flags), test.expect(c), OUTCOME_VERSION,
    ]).encode()).hexdigest()
    entry = CACHE / f"{key}.json"
    if use_cache and entry.exists():
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
        got = results[(t, c)][0]
        return got == "skip" or t.expect(c) is None or got == t.expect(c)

    failures, rows, differs = 0, [], []
    for t in tests:
        bad = [c for c in compilers if not matches(t, c)]
        failed = any(c.role == "baseline" for c in bad)
        failures += failed
        rows.append((t, "FAIL" if failed else "pass"))
        for c in bad:
            got, detail = results[(t, c)]
            line = f"{t.name} [{c.id}]: expected {t.expect(c)}, got {got}  {detail}"
            if c.role == "baseline":
                print("FAIL " + line)
            else:
                differs.append(line)

    for line in differs:
        print("differs " + line)
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
