#!/usr/bin/env python3
"""Run the clean-room front end's conformance suite and compare with a baseline.

The front end in frontend/ reads only Tidal's own files (decision 0029). A
change to the tests that makes a test it passed fail is a signal: either the
change disagrees with the front end's reading of the spec, or the front end
is out of date. The baseline is reports/clean-room/rust-frontend/conformance.tsv,
one line per test with its status.

Every run builds both front ends: the old one in frontend/ (tidal-fe) and
the new one in frontend/next/ (tidal-next, decision 0060). The takeover
list, reports/clean-room/rust-frontend/takeover.tsv, names the tests that
the new front end owns (decisions 0065 and 0066). For an owned test, the
status is the new front end's (`tidal-next conform -v`); for every other
test, it is the old front end's (`tidal-fe conform -v`). The comparison
with the baseline, --update, and the completion line use these statuses.
An owned test whose new status is not a passing status fails the check.

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
baseline, with the new front end's status for each owned test.

The last line gives the count that decision 0060's completion test
uses: the tests that fail their check, and the program tests under
tests/programs/ that do not run to their result. Both leave out the
tests that a row of the front-end ledger in state `out of scope` names
in its Record (decision 0061). The line then splits the tests that fail
their check by the marks file, reports/clean-room/rust-frontend/rewrite-marks.tsv
(decision 0063). A marked test needs the template model of
frontend/TEMPLATE-MODEL.md, and the rewrite takes it over. The rest count
toward the gate of decisions 0062 and 0063. The line keeps its earlier
numbers in their order, adds the two counts after the first, and ends
with the count of the takeover list:

    frontend: completion (decision 0060): N tests fail their check, of which
    M need the template model (marks file) and N-M count toward the gate of
    decisions 0062 and 0063; P/Q program tests do not run to their result;
    K tests out of scope; O owned by the new front end

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

With --next, the tool also measures the rewrite (decision 0064, item RW;
frontend/DESIGN-rewrite.md, section 5) after the lines above, and prints
nothing else differently:

    next: unit: K tests pass
    next: lex: T/N files give the old front end's tokens
    next: parse: N/M tests parse as their outcome needs
    next: parse: R of the D disagreements are reject tests that stop on the line of the old front end's first error
    next: shadow: step 3 plain: P/N pass their check, Q of the old front end's O
    next: shadow: step 3: P/N pass their check, Q of the old front end's O; R pass on the new front end only
    next: shadow: all: P/N pass their check; U unsupported
    next: owned: O tests (takeover list)

The unit line runs `cargo test` in frontend/next/; a failing unit test
fails the check. The lex line runs `lex` on both binaries for every test
file and extra translation unit and compares the dumps (kind, text, line,
literal, and the lexer's diagnostics). The parse line counts the tests
whose parse by `tidal-next parse` gives what the test's outcome needs: a
tree for each unit of an `accept`, `run`, `run-fail`, or `link-fail`
test, a tree for a `reject` test whose first error on the old front end
names a rule other than TR-12, and a stop for a `reject` test whose first
error names TR-12. A `reject` test with no error that names a rule is not
judged. The tool lists each disagreement with every place where the
grammar derives no form (`tidal-next parse --all`, which skips each
failed statement, member, or declaration and goes on), and a further
line counts the disagreements that are reject tests whose first stop is
on the line of the old front end's first error.

The shadow lines (build step 3) run `tidal-next conform tests -v` and
count the tests whose check the new front end passes, beside the old
front end's count on the same tests. "step 3" is the tests whose
`tidal-rules` lines name only rules of the pages of the foundations,
expressions, statements, and declarations chapters (the prefixes of the
page table in frontend/DESIGN-rewrite.md section 3). "step 3 plain" is
those of them whose source, comments and literals removed, uses no
keyword of a later step's pages. The tool lists each step 3 plain test
whose check the old front end passes and the new one does not. The
shadow run changes no status and no baseline.

The takeover list, reports/clean-room/rust-frontend/takeover.tsv, names
the tests that the new front end owns: a test path, the build step, and
the commit, separated by tabs. Every run reports a row whose test does
not exist, a row of the wrong form, a test listed twice, and an owned
test whose status on the new front end is not a passing status, on
lines that start with "frontend: takeover list:". Each of these problems
fails the check. With --next, the "next: takeover list:" lines report
them again, and the shadow lines compare with the old front end's status
of each test, owned or not.

Usage:
    tools/frontend_check.py [--update] [--next]
"""

import argparse
import concurrent.futures
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
NEXT = FRONTEND / "next"
TAKEOVER = ROOT / "reports" / "clean-room" / "rust-frontend" / "takeover.tsv"
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


def conform(cargo_bin: str, crate: Path, binary: str) -> tuple[dict[str, str], str]:
    """Build a front end's crate and return {test path: status} for every
    test, and the report, from `BINARY conform tests -v`. A test that the
    report does not list passes."""
    build = subprocess.run([cargo_bin, "build", "--release", "-q"], cwd=crate,
                           capture_output=True, text=True)
    if build.returncode:
        sys.exit(f"frontend: build of {crate.relative_to(ROOT)} failed\n{build.stderr}")
    out = subprocess.run([str(crate / "target" / "release" / binary), "conform",
                          str(ROOT / "tests"), "-v"],
                         cwd=crate, capture_output=True, text=True, errors="replace", check=True).stdout
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
    return status, out


def allow_no_diagnostic(status: dict[str, str]) -> dict[str, str]:
    """`MISSED-REJECT` becomes `NO-DIAGNOSTIC-ALLOWED` for a test whose
    cited rules are all `no-diagnostic` (decision 0061, item CT)."""
    tests = ROOT / "tests"
    no_diagnostic = {m.group(1) for f in SPEC.rglob("*.md")
                     for m in re.finditer(r"^:::\{rule\} (\S+)\n:kind: no-diagnostic$", f.read_text(), re.M)}
    out = dict(status)
    for name, s in status.items():
        if s != "MISSED-REJECT":
            continue
        rules = cited_rules(tests / name)
        if rules and rules <= no_diagnostic:
            out[name] = "NO-DIAGNOSTIC-ALLOWED"
    return out


def results(cargo_bin: str) -> tuple[dict[str, str], str]:
    """Return {test path: status} for every test, and the report, from
    `tidal-fe conform -v`."""
    status, out = conform(cargo_bin, FRONTEND, "tidal-fe")
    return allow_no_diagnostic(status), out


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
             "# compares with this file and rewrites it with --update. A test that",
             "# takeover.tsv lists has the new front end's status (tidal-next)."]
    lines += [f"{s}\t{n}" for n, s in sorted(status.items())]
    # Other agents run this tool, so a reader must never see half a file.
    with tempfile.NamedTemporaryFile("w", dir=BASELINE.parent, prefix=".conformance.",
                                     suffix=".tmp", delete=False) as f:
        f.write("\n".join(lines) + "\n")
    os.replace(f.name, BASELINE)


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--update", action="store_true", help="rewrite the baseline")
    ap.add_argument("--next", action="store_true", help="also measure the rewrite in frontend/next/")
    args = ap.parse_args()
    cargo_bin = cargo()
    if not cargo_bin:
        print("frontend: skipped (cargo not found, see documentation/setup.md)")
        return 0
    old_now, report = results(cargo_bin)
    # The new front end's status counts for each owned test (decision 0066).
    new_now, _ = conform(cargo_bin, NEXT, "tidal-next")
    owned, takeover_problems = read_takeover()
    now = dict(old_now)
    for t in owned:
        if t in new_now:
            now[t] = new_now[t]
    now = allow_no_diagnostic(now)
    for t in owned:
        if t in new_now and now[t] not in PASSING:
            takeover_problems.append(f"{t} is owned by the new front end, which gives {now[t]}")
    if args.update or not BASELINE.exists():
        for p in takeover_problems:
            print(f"frontend: takeover list: {p}")
        write_baseline(now)
        print(f"frontend: baseline written, {sum(s in PASSING for s in now.values())}/{len(now)} pass")
        return 1 if takeover_problems else 0
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
          f"{len(oos & set(now))} tests out of scope; {len(owned)} owned by the new front end")
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
    for p in takeover_problems:
        print(f"frontend: takeover list: {p}")
    next_failed = check_next(cargo_bin, old_now, new_now) if args.next else False
    return 1 if worse or takeover_problems or next_failed else 0


def header_expect(path: Path) -> tuple[str | None, list[str]]:
    """A test's base expectation and its extra translation units."""
    expect, sources = None, []
    for line in path.read_text(errors="replace").splitlines():
        if not line.startswith("//"):
            break
        if line.startswith("// tidal-expect:") and expect is None:
            words = line.split(":", 1)[1].split()
            expect = words[0] if words else ""
        if line.startswith("// tidal-sources:"):
            sources += line.split(":", 1)[1].split()
    return expect, sources


def batches(items: list, size: int = 400):
    for k in range(0, len(items), size):
        yield items[k:k + size]


def dumps(binary: Path, files: list[str]) -> dict[str, str]:
    """`BINARY lex` over the files, split by file."""
    out: dict[str, str] = {}
    for chunk in batches(files):
        text = subprocess.run([str(binary), "lex", *chunk], cwd=ROOT, capture_output=True, text=True,
                              errors="replace").stdout
        if len(chunk) == 1:
            out[chunk[0]] = text
            continue
        name = None
        for line in text.splitlines(keepends=True):
            if line.startswith("== "):
                name = line[3:].rstrip("\n")
                out[name] = ""
            elif name is not None:
                out[name] += line
    return out


def first_error(binary: Path, test: str) -> tuple[list[str], int] | None:
    """The rules and the line of the old front end's first error, or None."""
    r = subprocess.run([str(binary), "check", test], cwd=ROOT, capture_output=True, text=True,
                       errors="replace", timeout=120)
    m = re.search(r":(\d+): error \[([^\]]+)\]", r.stdout + r.stderr)
    return ([x.strip() for x in m.group(2).split(",")], int(m.group(1))) if m else None


def read_takeover() -> tuple[list[str], list[str]]:
    """The takeover list's tests and its problems."""
    owned: list[str] = []
    problems: list[str] = []
    if not TAKEOVER.exists():
        return owned, [f"{TAKEOVER.relative_to(ROOT)} does not exist"]
    for number, line in enumerate(TAKEOVER.read_text().splitlines(), 1):
        if not line.strip() or line.startswith("#"):
            continue
        cells = line.split("\t")
        if len(cells) != 3 or not all(c.strip() for c in cells):
            problems.append(f"line {number} does not have a test, a step, and a commit")
            continue
        test = cells[0].strip()
        if not (ROOT / "tests" / test).exists():
            problems.append(f"line {number}: {test} does not exist")
        if test in owned:
            problems.append(f"line {number}: {test} is listed twice")
        owned.append(test)
    return owned, problems


def check_next(cargo_bin: str, now: dict[str, str], new_now: dict[str, str]) -> bool:
    """The `next:` lines (decision 0064, item RW). Returns True when a unit
    test fails, the crate does not build, or the takeover list has a
    problem. `now` is the old front end's status for each test, and
    `new_now` the new front end's."""
    build = subprocess.run([cargo_bin, "build", "--release", "-q"], cwd=NEXT, capture_output=True, text=True)
    if build.returncode:
        print(f"next: build failed\n{build.stderr}")
        return True
    unit = subprocess.run([cargo_bin, "test", "--release", "-q"], cwd=NEXT, capture_output=True, text=True)
    passed = sum(int(m) for m in re.findall(r"test result: \w+\. (\d+) passed", unit.stdout))
    failed = sum(int(m) for m in re.findall(r"(\d+) failed", unit.stdout))
    if unit.returncode or failed:
        print(f"next: unit: {passed} tests pass, {failed} fail")
        for line in unit.stdout.splitlines():
            if line.startswith("---- ") or "panicked" in line:
                print(f"next: unit: {line}")
    else:
        print(f"next: unit: {passed} tests pass")
    old = FRONTEND / "target" / "release" / "tidal-fe"
    new = NEXT / "target" / "release" / "tidal-next"
    tests = ROOT / "tests"
    files = sorted(p.relative_to(ROOT).as_posix() for p in tests.rglob("*.cpp"))
    a, b = dumps(old, files), dumps(new, files)
    differ = [f for f in files if a.get(f) != b.get(f)]
    for f in differ[:20]:
        print(f"next: lex: {f.removeprefix('tests/')} differs")
    print(f"next: lex: {len(files) - len(differ)}/{len(files)} files give the old front end's tokens")
    # The parse count.
    main = [f for f in files if not f.endswith(".tu.cpp")]
    want: dict[str, str] = {}
    first_line: dict[str, int] = {}
    units: dict[str, list[str]] = {}
    rejects = []
    for f in main:
        expect, sources = header_expect(ROOT / f)
        units[f] = [f] + [(Path(f).parent / s).as_posix() for s in sources]
        if expect in ("accept", "run", "run-fail", "link-fail"):
            want[f] = "tree"
        elif expect == "reject":
            rejects.append(f)
    with concurrent.futures.ThreadPoolExecutor(max_workers=os.cpu_count() or 4) as pool:
        for f, found in zip(rejects, pool.map(lambda t: first_error(old, t), rejects)):
            if found is not None:
                want[f] = "stop" if "TR-12" in found[0] else "tree"
                first_line[f] = found[1]
    parsed: dict[str, str] = {}
    more: dict[str, list[str]] = {}
    for chunk in batches(sorted({u for f in want for u in units[f]})):
        text = subprocess.run([str(new), "parse", "--all", *chunk], cwd=ROOT, capture_output=True, text=True,
                              errors="replace").stdout
        for line in text.splitlines():
            status, _, rest = line.partition("\t")
            name, *others = rest.split("\t")
            parsed[name] = status
            more[name] = others
    agree = 0
    disagree = []
    same_line = 0
    for f in sorted(want):
        if want[f] == "tree":
            bad = [(u, parsed.get(u, "missing")) for u in units[f] if parsed.get(u) != "tree"]
            if not bad:
                agree += 1
            else:
                m = re.match(r"stop (\d+):", bad[0][1])
                if f in first_line and bad[0][0] == f and m and int(m.group(1)) == first_line[f]:
                    same_line += 1
                also = "".join(f"; also stops at {o}" for o in more.get(bad[0][0], []))
                disagree.append(f"{f}: wants a tree, {bad[0][0].removeprefix('tests/') if bad[0][0] != f else 'the unit'} gives {bad[0][1]}{also}")
        else:
            got = parsed.get(f, "missing")
            if got.startswith("stop"):
                agree += 1
            else:
                disagree.append(f"{f}: wants a stop (TR-12), gives {got}")
    for d in disagree:
        print(f"next: parse: {d.removeprefix('tests/')}")
    print(f"next: parse: {agree}/{len(want)} tests parse as their outcome needs")
    print(f"next: parse: {same_line} of the {len(disagree)} disagreements are reject tests that stop on the line "
          f"of the old front end's first error")
    shadow = shadow_lines(new_now, now)
    owned, problems = read_takeover()
    for t in owned:
        if t in shadow and shadow[t] not in PASSING:
            problems.append(f"{t} is owned by the new front end, which gives {shadow[t]}")
    for p in problems:
        print(f"next: takeover list: {p}")
    print(f"next: owned: {len(owned)} tests (takeover list)")
    return bool(unit.returncode or failed or problems)


STEP_CHAPTERS = {"3": {"foundations", "expressions", "statements", "declarations"},
                 "4": {"classes", "overloading", "exceptions"}}
LATER = re.compile(r"\b(class|struct|union|template|typename|concept|requires|throw|try|catch|operator|"
                   r"virtual|friend|this|new|delete|std)\b|#\s*include")


def page_prefixes() -> dict[str, str]:
    """{rule prefix: chapter}, from the page table of DESIGN-rewrite.md
    section 3."""
    out: dict[str, str] = {}
    design = FRONTEND / "DESIGN-rewrite.md"
    for line in design.read_text().splitlines():
        m = re.match(r"^\| `([a-z-]+)/[^`]+` \| `[^`]+` \| ([A-Z, ]+) \|", line)
        if m:
            for prefix in m.group(2).split(","):
                out[prefix.strip()] = m.group(1)
    return out


def code_text(path: Path) -> str:
    """A source file with its comments and literals removed."""
    t = path.read_text(errors="replace")
    t = re.sub(r"//[^\n]*|/\*.*?\*/", " ", t, flags=re.S)
    return re.sub(r'"(\\.|[^"\\\n])*"|\'(\\.|[^\'\\\n])*\'', " ", t)


def shadow_lines(status: dict[str, str], base: dict[str, str]) -> dict[str, str]:
    """The shadow lines of build step 3: they count the new front end's
    status of each test by scope, and return the status."""
    tests = ROOT / "tests"
    prefixes = page_prefixes()
    step3, plain = [], []
    for name in sorted(status):
        path = tests / name
        rules = cited_rules(path)
        chapters = {prefixes.get(r.split("-")[0]) for r in rules}
        if not rules or not chapters <= STEP_CHAPTERS["3"]:
            continue
        step3.append(name)
        sources = [path] + [path.parent / s for s in header_expect(path)[1]]
        if not any(LATER.search(code_text(s)) for s in sources if s.exists()):
            plain.append(name)
    ok_new = lambda n: rank(status.get(n)) >= 1
    ok_old = lambda n: rank(base.get(n)) >= 1
    for n in plain:
        if ok_old(n) and not ok_new(n):
            print(f"next: shadow: {n}: {status[n]}, the old front end {base.get(n)}")
    for label, scope in (("step 3 plain", plain), ("step 3", step3)):
        old = [n for n in scope if ok_old(n)]
        line = (f"next: shadow: {label}: {sum(ok_new(n) for n in scope)}/{len(scope)} pass their check, "
                f"{sum(ok_new(n) for n in old)} of the old front end's {len(old)}")
        if label == "step 3":
            line += f"; {sum(ok_new(n) and not ok_old(n) for n in scope)} pass on the new front end only"
        print(line)
    print(f"next: shadow: all: {sum(ok_new(n) for n in status)}/{len(status)} pass their check; "
          f"{sum(s == 'unsupported' for s in status.values())} unsupported")
    return status


if __name__ == "__main__":
    sys.exit(main())
