#!/usr/bin/env python3
"""Run Tidal conformance tests on Clang and GCC.

Each test is a .cpp file under tests/ with directive comments at the top
(see docs/spec-format.md). The runner compiles each test with every
compiler, runs it if the test expects `run`, and compares the outcome with
the expectation.

Usage:
    tools/run_tests.py [paths...] [--report tests/RESULTS.md] [-j N]
"""

import argparse
import concurrent.futures
import os
import re
import subprocess
import sys
from dataclasses import dataclass, field
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
TESTS = ROOT / "tests"
BUILD = ROOT / "build"

COMPILERS = {
    "clang": "clang++",
    "gcc": "g++",
}
BASE_FLAGS = ["-std=c++23", "-pedantic-errors"]
EXPECTATIONS = {"accept", "reject", "run"}
DIRECTIVE = re.compile(r"^//\s*tidal-([a-z-]+):\s*(.*?)\s*$")


@dataclass(eq=False)
class Test:
    path: Path
    rules: list[str]
    expect: dict[str, str]
    flags: list[str] = field(default_factory=list)
    notes: list[str] = field(default_factory=list)

    @property
    def name(self) -> str:
        return str(self.path.relative_to(TESTS))


def parse_test(path: Path) -> Test:
    rules, flags, notes = [], [], []
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
        else:
            raise ValueError(f"{path}: unknown directive tidal-{key}")
    if not rules:
        raise ValueError(f"{path}: missing tidal-rules")
    if default is None and set(overrides) != set(COMPILERS):
        raise ValueError(f"{path}: missing tidal-expect")
    expect = {c: overrides.get(c, default) for c in COMPILERS}
    for value in expect.values():
        if value not in EXPECTATIONS:
            raise ValueError(f"{path}: bad expectation {value!r}")
    return Test(path, rules, expect, flags, notes)


def outcome(test: Test, compiler: str) -> tuple[str, str]:
    """Return (outcome, detail). Outcome is accept, reject, run, or run-fail."""
    out = BUILD / compiler / test.path.relative_to(TESTS).with_suffix("")
    out.parent.mkdir(parents=True, exist_ok=True)
    wants_run = test.expect[compiler] == "run"
    cmd = [COMPILERS[compiler], *BASE_FLAGS, *test.flags, str(test.path), "-o", str(out)]
    if not wants_run:
        cmd.insert(1, "-c")
    proc = subprocess.run(cmd, capture_output=True, text=True)
    if proc.returncode != 0:
        diags = [l for l in proc.stderr.splitlines() if ": error: " in l or ": note: " in l]
        diags = [l for l in diags if "in call to" not in l and "declared here" not in l]
        return "reject", " / ".join(diags[:2]) or proc.stderr.strip()[:200]
    if not wants_run:
        return "accept", ""
    try:
        run = subprocess.run([str(out)], capture_output=True, text=True, timeout=10)
    except subprocess.TimeoutExpired:
        return "run-fail", "timeout"
    if run.returncode != 0:
        return "run-fail", f"exit {run.returncode}: {(run.stdout + run.stderr).strip()[:200]}"
    return "run", ""


def compiler_version(compiler: str) -> str:
    proc = subprocess.run([COMPILERS[compiler], "--version"], capture_output=True, text=True)
    return proc.stdout.splitlines()[0]


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("paths", nargs="*", type=Path, default=[TESTS])
    ap.add_argument("--report", type=Path, help="write a Markdown results table")
    ap.add_argument("-j", type=int, default=os.cpu_count())
    args = ap.parse_args()

    files = []
    for p in args.paths:
        p = p.resolve()
        files += sorted(p.rglob("*.cpp")) if p.is_dir() else [p]
    tests = [parse_test(f) for f in files]

    jobs = [(t, c) for t in tests for c in COMPILERS]
    with concurrent.futures.ThreadPoolExecutor(args.j) as pool:
        results = dict(zip(jobs, pool.map(lambda job: outcome(*job), jobs)))

    failures = 0
    rows = []
    for t in tests:
        got = {c: results[(t, c)] for c in COMPILERS}
        ok = {c: got[c][0] == t.expect[c] for c in COMPILERS}
        status = "pass" if all(ok.values()) else "FAIL"
        failures += status == "FAIL"
        rows.append((t, got, status))
        if status == "FAIL":
            print(f"FAIL {t.name}")
            for c in COMPILERS:
                mark = "ok " if ok[c] else "BAD"
                print(f"  {mark} {c}: expected {t.expect[c]}, got {got[c][0]}  {got[c][1]}")

    print(f"{len(tests) - failures}/{len(tests)} tests pass")

    if args.report:
        lines = [
            "# Test results",
            "",
            "Generated by `tools/run_tests.py --report`. Do not edit by hand.",
            "",
            *(f"- {c}: `{compiler_version(c)}`" for c in COMPILERS),
            f"- flags: `{' '.join(BASE_FLAGS)}`",
            "",
            "| Test | Rules | Expected | " + " | ".join(COMPILERS) + " | Status |",
            "|---|---|---|" + "---|" * len(COMPILERS) + "---|",
        ]
        for t, got, status in rows:
            expected = t.expect["clang"] if len(set(t.expect.values())) == 1 else ", ".join(
                f"{c}: {e}" for c, e in t.expect.items()
            )
            cells = " | ".join(got[c][0] for c in COMPILERS)
            lines.append(f"| `{t.name}` | {' '.join(t.rules)} | {expected} | {cells} | {status} |")
        lines += ["", f"{len(tests) - failures}/{len(tests)} tests pass.", ""]
        lines += [
            "## Rejections",
            "",
            "First error from each compiler that rejected a test, so a reviewer can",
            "check that the rejection has the intended cause.",
            "",
        ]
        for t, got, _ in rows:
            for c in COMPILERS:
                if got[c][0] == "reject":
                    detail = re.sub(r"\S+:\d+:\d+: ", "", got[c][1]).replace("|", "\\|")
                    lines.append(f"- `{t.name}` ({c}): {detail}")
        lines.append("")
        args.report.write_text("\n".join(lines))

    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
