#!/usr/bin/env python3
"""Run the clean-room front end's conformance suite and compare with a baseline.

The front end in frontend/ reads only Tidal's own files (decision 0029). A
change to the tests that makes a test it passed fail is a signal: either the
change disagrees with the front end's reading of the spec, or the front end
is out of date. The baseline is reports/clean-room/rust-frontend/conformance.tsv,
one line per test with its status.

A test passes when the front end's answer matches the test's base
expectation (`tidal-expect`). The passing statuses:

- `pass`.
- `EDG-ONLY`: the answer matches the base expectation, and the test's EDG
  override differs.
- `EDG-ALLOWED`: the answer matches only the EDG override, and the rules
  allow a second answer. A test with a `tidal-allows-edg: RULE-ID` line
  names the rules that allow it, and each named rule is impl-defined,
  unspecified, no-diagnostic, or conditionally-supported. A test with no
  such line needs every rule that it cites to be of one of those kinds.
- `pass-partial`: the run passes on every explored choice, but the
  exploration is partial (the bound stopped it, or a run was unsupported).
- `NO-DIAGNOSTIC-ALLOWED`: the front end accepts a test that expects a
  reject, and every rule that the test cites is `no-diagnostic`, so an
  implementation may accept it (decision 0061, item CT).

`EDG-AGREES` is a failed check: the answer matches only the EDG override,
and the rules do not allow that answer.

`NO-BASE` is a failed check: the test has no base `tidal-expect` line, so
the harness has no answer to judge by.

For a test that expects a run, `tidal-fe conform -v` gives the run's outcome
once the check passed: `run-unsupported`, `RUN-WRONG-RESULT`, `RUN-MISSED-UB`,
`RUN-UNEXPECTED-UB`, or `RUN-OTHER-OUTCOME` (one explored run gives an
outcome that the expectation does not allow). Such a status counts as a
passed check and a failed run. Each status has a rank: 2 for a full pass,
1 for a passed check with a failed run, 0 for a failed check.

An `accept` test with a `tidal-interpreter: run-fail` line (decision 0053)
also runs in the interpreter, and its run must stop at undefined behavior
that names one of the test's rules. Its status is a run status as above.
The summary counts these tests and how many of them pass.

Fails when a test's status changes to a failing status: a move from a
passing status, and a move from one failing status to another, both fail.
New tests and tests that improved are reported. --update rewrites the
baseline.

The last line gives the count that decision 0060's completion test
uses: the tests that fail their check, and the program tests under
tests/programs/ that do not run to their result. Both leave out the
tests that a row of the front-end ledger in state `out of scope` names
in its Record (decision 0061). The line then splits the tests that fail
their check by the marks file, reports/clean-room/rust-frontend/rewrite-marks.tsv
(decision 0063). A marked test needs the template model of
frontend/TEMPLATE-MODEL.md, and the rewrite takes it over. The rest count
toward the gate of decisions 0062 and 0063. The line keeps its earlier
numbers in their order and adds the two counts after the first:

    frontend: completion (decision 0060): N tests fail their check, of which
    M need the template model (marks file) and N-M count toward the gate of
    decisions 0062 and 0063; P/Q program tests do not run to their result;
    K tests out of scope

A row of the marks file is a test path under tests/, the section of
TEMPLATE-MODEL.md that passes the test, and a cause, separated by tabs.
The tool reports a marked test that does not exist, that passes its
check, or that the ledger marks out of scope, and a row whose section
TEMPLATE-MODEL.md does not have. These reports do not change the exit
status.

A baseline row is a status and a test path, separated by a tab; later
columns, if a row has any, are ignored. --update writes the baseline to
a temporary file and renames it, so that another process that runs the
tool never reads a part of the file.

Usage:
    tools/frontend_check.py [--update]
"""

import argparse
import os
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
FRONTEND = ROOT / "frontend"
BASELINE = ROOT / "reports" / "clean-room" / "rust-frontend" / "conformance.tsv"
PASSING = {"pass", "EDG-ONLY", "EDG-ALLOWED", "pass-partial", "NO-DIAGNOSTIC-ALLOWED"}
LEDGER = ROOT / "reports" / "clean-room" / "rust-frontend" / "ledger.md"
MARKS = ROOT / "reports" / "clean-room" / "rust-frontend" / "rewrite-marks.tsv"
MODEL = FRONTEND / "TEMPLATE-MODEL.md"
SPEC = ROOT / "specification"
INTERPRETER = re.compile(r"^// tidal-interpreter:\s*run-fail\s*$", re.M)


def rank(status: str | None) -> int:
    """2 for a full pass, 1 for a passed check whose run failed, 0 otherwise."""
    if status in PASSING:
        return 2
    if status and status.lower().startswith("run-"):
        return 1
    return 0


def cargo() -> str | None:
    found = shutil.which("cargo")
    home = Path(os.environ.get("CARGO_HOME", Path.home() / ".cargo")) / "bin" / "cargo"
    return found or (str(home) if home.exists() else None)


def results(cargo_bin: str) -> tuple[dict[str, str], str]:
    """Return {test path: status} for every test, and the report, from
    `tidal-fe conform -v`."""
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
    no_diagnostic = {m.group(1) for f in SPEC.rglob("*.md")
                     for m in re.finditer(r"^:::\{rule\} (\S+)\n:kind: no-diagnostic$", f.read_text(), re.M)}
    for name, s in status.items():
        rules = cited_rules(tests / name)
        if s == "MISSED-REJECT" and rules and rules <= no_diagnostic:
            status[name] = "NO-DIAGNOSTIC-ALLOWED"
    return status, out


def cited_rules(path: Path) -> set[str]:
    """The rule IDs of a test's tidal-rules lines."""
    rules: set[str] = set()
    for line in path.read_text().splitlines():
        if not line.startswith("//"):
            break
        if line.startswith("// tidal-rules:"):
            rules |= set(line.split(":", 1)[1].split())
    return rules


def out_of_scope() -> set[str]:
    """The tests that a front-end ledger row in state `out of scope` names."""
    found: set[str] = set()
    for line in LEDGER.read_text().splitlines():
        if m := re.match(r"^\| G\d+ \| [^|]+ \| out of scope \| ([^|]*)\|", line):
            found |= set(re.findall(r"`([^`]+\.cpp)`", m.group(1)))
    return found


def rewrite_marks() -> tuple[dict[str, str], list[str]]:
    """The marks file: {test path: section of TEMPLATE-MODEL.md}, and its
    problems (decision 0063)."""
    marks: dict[str, str] = {}
    problems: list[str] = []
    if not MARKS.exists():
        return marks, [f"{MARKS.relative_to(ROOT)} does not exist"]
    sections = set(re.findall(r"^## (\d+)\.", MODEL.read_text(), re.M)) if MODEL.exists() else set()
    for number, line in enumerate(MARKS.read_text().splitlines(), 1):
        if not line.strip() or line.startswith("#"):
            continue
        cells = line.split("\t")
        if len(cells) != 3 or not all(c.strip() for c in cells):
            problems.append(f"line {number} does not have a test, a section, and a cause")
            continue
        test, section = cells[0].strip(), cells[1].strip()
        if section not in sections:
            problems.append(f"line {number}: {MODEL.name} has no section {section}")
        if test in marks:
            problems.append(f"line {number}: {test} is marked twice")
        marks[test] = section
    return marks, problems


def interpreter_tests() -> list[str]:
    """The tests whose run only the interpreter checks (decision 0053)."""
    tests = ROOT / "tests"
    found = []
    for path in sorted(tests.rglob("*.cpp")):
        header = []
        for line in path.read_text().splitlines():
            if not line.startswith("//"):
                break
            header.append(line)
        if INTERPRETER.search("\n".join(header)):
            found.append(path.relative_to(tests).as_posix())
    return found


def read_baseline() -> dict[str, str]:
    base = {}
    for line in BASELINE.read_text().splitlines():
        if line and not line.startswith("#"):
            status, name = line.split("\t")[:2]
            base[name] = status
    return base


def write_baseline(status: dict[str, str]) -> None:
    lines = ["# The clean-room front end's result for each test. tools/frontend_check.py",
             "# compares with this file and rewrites it with --update."]
    lines += [f"{s}\t{n}" for n, s in sorted(status.items())]
    # Other agents run this tool, so a reader must never see half a file.
    with tempfile.NamedTemporaryFile("w", dir=BASELINE.parent, prefix=".conformance.",
                                     suffix=".tmp", delete=False) as f:
        f.write("\n".join(lines) + "\n")
    os.replace(f.name, BASELINE)


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--update", action="store_true", help="rewrite the baseline")
    args = ap.parse_args()
    cargo_bin = cargo()
    if not cargo_bin:
        print("frontend: skipped (cargo not found, see documentation/setup.md)")
        return 0
    now, report = results(cargo_bin)
    if args.update or not BASELINE.exists():
        write_baseline(now)
        print(f"frontend: baseline written, {sum(s in PASSING for s in now.values())}/{len(now)} pass")
        return 0
    base = read_baseline()
    # Any move to a failing status fails, from a pass or from another
    # failing status.
    worse = sorted(n for n, s in now.items() if n in base and s != base[n] and s not in PASSING)
    better = sorted(n for n, s in now.items() if n in base and s in PASSING
                    and base[n] not in PASSING)
    # A move between two passing statuses, for example to `pass-partial`.
    moved = sorted(n for n, s in now.items() if n in base and s != base[n] and s in PASSING
                   and base[n] in PASSING)
    new_failing = sorted(n for n, s in now.items() if n not in base and s not in PASSING)
    new = sum(n not in base for n in now)
    for n in worse:
        print(f"frontend: {n}: {now[n]}, {base[n]} in the baseline")
    for n in better:
        print(f"frontend: {n}: {now[n]}, {base[n]} in the baseline")
    for n in moved:
        print(f"frontend: {n}: {now[n]}, {base[n]} in the baseline")
    for n in new_failing:
        print(f"frontend: new test {n}: {now[n]}")
    passing = sum(s in PASSING for s in now.values())
    checks = sum(rank(s) >= 1 for s in now.values())
    interp = interpreter_tests()
    print(f"frontend: {passing}/{len(now)} pass, {checks}/{len(now)} pass their check, "
          f"{len(worse)} worse, {len(better)} better, {len(moved)} moved between passing statuses, {new} new ({len(new_failing)} not passing)")
    print(f"frontend: {sum(now.get(n) in PASSING for n in interp)}/{len(interp)} "
          f"interpreter-only expectations met")
    oos = out_of_scope()
    unknown = sorted(oos - set(now))
    if unknown:
        print(f"frontend: out-of-scope rows name tests that do not exist: {', '.join(unknown)}")
    marks, problems = rewrite_marks()
    for p in problems:
        print(f"frontend: marks file: {p}")
    for n in sorted(marks):
        if n not in now:
            print(f"frontend: marks file: {n} does not exist")
        elif n in oos:
            print(f"frontend: marks file: {n} is also out of scope in the ledger")
        elif rank(now[n]) > 0:
            print(f"frontend: marks file: {n} passes its check ({now[n]}); remove its row")
    failing = [n for n, s in now.items() if rank(s) == 0 and n not in oos]
    marked = sum(n in marks for n in failing)
    programs = [n for n in now if n.startswith("programs/") and n not in oos]
    not_run = sum(rank(now[n]) < 2 for n in programs)
    print(f"frontend: completion (decision 0060): {len(failing)} tests fail their check, "
          f"of which {marked} need the template model (marks file) and "
          f"{len(failing) - marked} count toward the gate of decisions 0062 and 0063; "
          f"{not_run}/{len(programs)} program tests do not run to their result; "
          f"{len(oos & set(now))} tests out of scope")
    no_base = sorted(n for n, s in now.items() if s == "NO-BASE")
    if no_base:
        print(f"frontend: {len(no_base)} tests with no base expectation: {', '.join(no_base)}")
    # The last row of `tidal-fe conform`'s table of rule IDs per page.
    rule_rows = [ln for ln in report.splitlines()
                 if ln.startswith("| **total** |") and ln.count("|") == 8]
    if rule_rows:
        cells = [c.strip() for c in rule_rows[-1].strip("|").split("|")]
        print(f"frontend: {int(cells[2]) + int(cells[3])}/{cells[1]} passing rejects name a "
              f"cited rule ({cells[2]}) or a linked rule ({cells[3]}) in their first error; "
              f"`tidal-fe conform` gives the table per page")
    return 1 if worse else 0


if __name__ == "__main__":
    sys.exit(main())
