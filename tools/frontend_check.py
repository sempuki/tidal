#!/usr/bin/env python3
"""Run the clean-room front end's conformance suite and compare with a baseline.

The front end in frontend/ reads only Tidal's own files (decision 0029). A
change to the tests that makes a test it passed fail is a signal: either the
change disagrees with the front end's reading of the spec, or the front end
is out of date. The baseline is reports/clean-room/rust-frontend/conformance.tsv,
one line per test with its status.

A test passes when the front end's answer matches the test's expectation.
"Differs from EDG only" counts as a pass, so either answer is accepted on a
test with an EDG override.

Fails when a test that passed in the baseline does not pass now. New tests
and tests that improved are reported. --update rewrites the baseline.

Usage:
    tools/frontend_check.py [--update]
"""

import argparse
import os
import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
FRONTEND = ROOT / "frontend"
BASELINE = ROOT / "reports" / "clean-room" / "rust-frontend" / "conformance.tsv"
PASSING = {"pass", "EDG-ONLY"}


def cargo() -> str | None:
    found = shutil.which("cargo")
    home = Path(os.environ.get("CARGO_HOME", Path.home() / ".cargo")) / "bin" / "cargo"
    return found or (str(home) if home.exists() else None)


def results(cargo_bin: str) -> dict[str, str]:
    """Return {test path: status} for every test, from `tidal-fe conform -v`."""
    build = subprocess.run([cargo_bin, "build", "--release", "-q"], cwd=FRONTEND,
                           capture_output=True, text=True)
    if build.returncode:
        sys.exit(f"frontend: build failed\n{build.stderr}")
    out = subprocess.run([str(FRONTEND / "target" / "release" / "tidal-fe"), "conform",
                          str(ROOT / "tests"), "-v"],
                         cwd=FRONTEND, capture_output=True, text=True, check=True).stdout
    status = {}
    for line in out.splitlines():
        parts = line.split("\t")
        if len(parts) >= 2 and parts[1].endswith(".cpp"):
            status[parts[1]] = parts[0]
    tests = ROOT / "tests"
    for path in tests.rglob("*.cpp"):
        name = path.relative_to(tests).as_posix()
        if not name.endswith(".tu.cpp"):
            status.setdefault(name, "pass")
    return status


def read_baseline() -> dict[str, str]:
    base = {}
    for line in BASELINE.read_text().splitlines():
        if line and not line.startswith("#"):
            status, name = line.split("\t")
            base[name] = status
    return base


def write_baseline(status: dict[str, str]) -> None:
    lines = ["# The clean-room front end's result for each test. tools/frontend_check.py",
             "# compares with this file and rewrites it with --update."]
    lines += [f"{s}\t{n}" for n, s in sorted(status.items())]
    BASELINE.write_text("\n".join(lines) + "\n")


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--update", action="store_true", help="rewrite the baseline")
    args = ap.parse_args()
    cargo_bin = cargo()
    if not cargo_bin:
        print("frontend: skipped (cargo not found, see documentation/setup.md)")
        return 0
    now = results(cargo_bin)
    if args.update or not BASELINE.exists():
        write_baseline(now)
        print(f"frontend: baseline written, {sum(s in PASSING for s in now.values())}/{len(now)} pass")
        return 0
    base = read_baseline()
    worse = sorted(n for n, s in now.items() if s not in PASSING and base.get(n) in PASSING)
    better = sorted(n for n, s in now.items() if s in PASSING and n in base and base[n] not in PASSING)
    new_failing = sorted(n for n, s in now.items() if n not in base and s not in PASSING)
    new = sum(n not in base for n in now)
    for n in worse:
        print(f"frontend: {n}: {now[n]}, passed in the baseline")
    for n in better:
        print(f"frontend: {n}: passes now, {base[n]} in the baseline")
    for n in new_failing:
        print(f"frontend: new test {n}: {now[n]}")
    passing = sum(s in PASSING for s in now.values())
    print(f"frontend: {passing}/{len(now)} pass, {len(worse)} worse, {len(better)} better, "
          f"{new} new ({len(new_failing)} not passing)")
    return 1 if worse else 0


if __name__ == "__main__":
    sys.exit(main())
