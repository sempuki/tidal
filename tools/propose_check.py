#!/usr/bin/env python3
"""Check the change tables of Propose reports against the specification.

A Propose report (documentation/exercises.md, section 4) lists the rules
its proposal changes in the table of its section 2. The specification
moves on after the report is written, so a row can go stale. The tool
reads the change table of each reports/clean-room/X*/report.md and
prints one line for each row that conflicts with the specification now:

- withdrawn: the row names a rule that is now withdrawn. The line names
  the rules that the withdrawn directive points to ("merged into CE-8").
- class: the row's old class differs from the rule's class now.
- new-id: a row of kind "new" has an ID that a page now defines. A new
  rule takes an ID of the form X<n>:new-<k>, which no page can define.
- unknown: a row that is not "new" names a rule or principle that no
  page defines.

A row whose first cell is not a rule or principle ID (a grammar
production, a glossary term, "new VC rule") is not checked.

The tool reads a table whose header names the ID column "ID" or "Rule
ID", the kind column "Change" or "Kind of change", and an "Old class"
column. When a table has both "Kind of change" and "Change", the first
gives the kind. The tool prints a warning for each table in section 2
that it cannot read this way, and for a report whose section 2 gives no
row.

A report can name the commit it read with a backquoted commit ID in its
header, before the first "## " heading (for example "Commit read:
`3d98c93`."). For such a report the tool also prints how many later
commits change specification/, and one "changed since <commit>" line for
each row whose rule or principle has other directive text at HEAD than
at that commit. The comparison leaves out the `:tests:` option. A
changed row is a fact for the reader of the report, and the tool does
not count it as a conflict.

The tool exits 0 even when it finds conflicts, because a report is a
record of its date and the check must not fail a commit. --strict makes
it exit 1 when it finds a conflict.

The rule IDs and classes come from parse_spec() in
tools/check_specification.py.

--corpus prints the size of the program corpus under tests/programs/
and exits. A program is a .cpp file that is not an extra unit
(.tu.cpp). The count splits the programs of the Write suites from the
programmer sketches under roadmap/, and counts the lines of every .cpp
and .h file, extra units and headers included. A Propose brief quotes
these numbers.

Usage:
    tools/propose_check.py [--strict] [REPORT ...]
    tools/propose_check.py --corpus
"""

import argparse
import re
import subprocess
import sys
from collections import Counter
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from check_specification import (  # noqa: E402
    CLASSES, DIRECTIVE, PRINCIPLE, ROOT, SPEC, parse_spec,
)

REPORTS = ROOT / "reports" / "clean-room"
CHANGES = re.compile(r"^## 2\.")
NEXT = re.compile(r"^## ")
ROW_ID = re.compile(r"^[A-Z]+-P?\d+$")
EXERCISE_ID = re.compile(r"^X\d+:new-\d+$")
RULE_REF = re.compile(r"\{rule\}`([A-Z]+-\d+)`")
COMMIT = re.compile(r"`([0-9a-f]{7,40})`")
KINDS = ["withdrawn", "class", "new-id", "unknown"]
ID_HEADERS = ["id", "rule id"]
KIND_HEADERS = ["kind of change", "change"]
OLD_HEADERS = ["old class"]


def shown(path: Path) -> Path:
    return path.relative_to(ROOT) if path.is_relative_to(ROOT) else path


def cells(line: str) -> list[str]:
    return [c.strip() for c in line.strip().strip("|").split("|")]


def column(header: list[str], names: list[str]) -> int | None:
    """Return the index of the first of names that the header has."""
    for name in names:
        if name in header:
            return header.index(name)
    return None


def read_tables(report: Path):
    """Return (rows, unread) for section 2 of a report.

    rows holds (line number, ID, change, old class) for each row of a
    change table. unread holds the line number and header of each table
    the tool cannot read.
    """
    rows, unread = [], []
    inside = False
    header = None
    columns = None
    for n, line in enumerate(report.read_text().splitlines(), 1):
        if CHANGES.match(line):
            inside = True
            continue
        if inside and NEXT.match(line):
            break
        if not inside:
            continue
        if not line.startswith("|"):
            header = None
            continue
        row = cells(line)
        if header is None:
            header = [c.lower() for c in row]
            columns = [column(header, names) for names in (ID_HEADERS, KIND_HEADERS, OLD_HEADERS)]
            if None in columns:
                columns = None
                unread.append((n, line.strip()))
            continue
        if set(line) <= set("|-: ") or columns is None:
            continue
        get = [row[i] if i < len(row) else "" for i in columns]
        rows.append((n, get[0].strip("`"), get[1], get[2]))
    return rows, unread


def change_rows(report: Path):
    """Yield (line number, ID, change, old class) for each row of the section 2 tables."""
    yield from read_tables(report)[0]


def merge_targets() -> dict[str, list[str]]:
    """Map each withdrawn rule to the rules its directive body names."""
    targets = {}
    for page in sorted(SPEC.rglob("*.md")):
        rule = None
        withdrawn = False
        body = []
        for line in page.read_text().splitlines():
            if m := DIRECTIVE.match(line):
                rule, withdrawn, body = m.group(1), False, []
            elif rule and line.startswith(":::"):
                if withdrawn:
                    targets[rule] = RULE_REF.findall(" ".join(body))
                rule = None
            elif rule and line.strip() == ":kind: withdrawn":
                withdrawn = True
            elif rule:
                body.append(line)
    return targets


def principle_ids() -> set[str]:
    ids = set()
    for page in SPEC.rglob("*.md"):
        for line in page.read_text().splitlines():
            if m := PRINCIPLE.match(line):
                ids.add(m.group(1))
    return ids


def report_commit(report: Path) -> tuple[str | None, str]:
    """Return the commit the report read and a note on how far specification/ has moved since."""
    head = []
    for line in report.read_text().splitlines()[1:]:
        if NEXT.match(line):
            break
        head.append(line)
    for sha in COMMIT.findall("\n".join(head)):
        run = subprocess.run(
            ["git", "rev-list", "--count", f"{sha}..HEAD", "--", "specification"],
            cwd=ROOT, capture_output=True, text=True)
        if run.returncode == 0:
            return sha, f"read {sha}, {run.stdout.strip()} later commits change specification/"
    return None, "names no commit"


def directive_texts(pages: dict[str, str]) -> dict[str, str]:
    """Map each rule and principle ID to its directive text without the :tests: option."""
    texts = {}
    for text in pages.values():
        current, body = None, []
        for line in text.splitlines():
            if m := DIRECTIVE.match(line) or PRINCIPLE.match(line):
                current, body = m.group(1), []
            elif current and line.startswith(":::"):
                texts[current] = "\n".join(body).strip()
                current = None
            elif current and not line.startswith(":tests:"):
                body.append(line)
    return texts


def pages_at(sha: str) -> dict[str, str]:
    """Return the text of each specification page at a commit."""
    names = subprocess.run(
        ["git", "ls-tree", "-r", "--name-only", sha, "--", "specification"],
        cwd=ROOT, capture_output=True, text=True, check=True).stdout.split()
    return {name: subprocess.run(["git", "show", f"{sha}:{name}"], cwd=ROOT,
                                 capture_output=True, text=True, check=True).stdout
            for name in names if name.endswith(".md")}


def changed_since(report: Path, sha: str, now: dict[str, str], cache: dict) -> list[str]:
    """Return one line for each row whose directive text differs between sha and HEAD."""
    if sha not in cache:
        cache[sha] = directive_texts(pages_at(sha))
    then = cache[sha]
    lines = []
    for n, rid, _, _ in change_rows(report):
        if rid in then and rid in now and then[rid] != now[rid]:
            lines.append(f"{shown(report)}:{n}: {rid}: changed since {sha}")
    return lines


def check(report: Path, rules: dict, principles: set[str], targets: dict) -> list[tuple[str, str]]:
    """Return (kind, line) for each conflict in one report."""
    name = shown(report)
    found = []
    for n, rid, change, old in change_rows(report):
        if EXERCISE_ID.match(rid) or not ROW_ID.match(rid):
            continue
        where = f"{name}:{n}: {rid}"
        known = rid in rules or rid in principles
        if change.lower() == "new":
            if known:
                found.append(("new-id", f"{where}: a new row uses an ID that a page now defines; "
                              f"use the X<n>:new-<k> form"))
            continue
        if not known:
            found.append(("unknown", f"{where}: no page defines this ID"))
            continue
        if rid in principles:
            continue
        current = rules[rid][1]
        if current == "withdrawn":
            into = targets.get(rid) or []
            where_to = ("merged into " + ", ".join(into)) if into else "no merge target given"
            found.append(("withdrawn", f"{where}: withdrawn ({where_to})"))
            continue
        before = old.split()[0].strip("`") if old.split() else ""
        if before in CLASSES and before != current:
            found.append(("class", f"{where}: old class {before}, now {current}"))
    return found


def corpus() -> None:
    """Print the size of the program corpus under tests/programs/."""
    base = ROOT / "tests" / "programs"
    files = sorted(p for p in base.rglob("*") if p.suffix in (".cpp", ".h"))
    programs = [p for p in files if p.suffix == ".cpp" and not p.name.endswith(".tu.cpp")]
    sketches = [p for p in programs if p.relative_to(base).parts[0] == "roadmap"]
    several = [p for p in programs if "tidal-sources:" in p.read_text()]
    lines = sum(len(p.read_text().splitlines()) for p in files)
    print(f"{len(programs)} programs: {len(programs) - len(sketches)} from the Write suites, "
          f"{len(sketches)} programmer sketches under roadmap/")
    print(f"{len(several)} programs have more than one unit")
    print(f"{lines} lines in {len(files)} files, with extra units and headers")


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("reports", nargs="*", type=Path,
                    help="reports to check (default: reports/clean-room/X*/report.md)")
    ap.add_argument("--strict", action="store_true",
                    help="exit 1 when a row conflicts with the specification")
    ap.add_argument("--corpus", action="store_true",
                    help="print the size of the program corpus and exit")
    args = ap.parse_args()
    if args.corpus:
        corpus()
        return 0

    reports = [p.resolve() for p in args.reports] or sorted(
        REPORTS.glob("X*/report.md"), key=lambda p: int(p.parent.name[1:]))
    rules, _ = parse_spec()
    principles = principle_ids()
    targets = merge_targets()

    now = directive_texts({str(p): p.read_text() for p in SPEC.rglob("*.md")})
    cache = {}
    totals = Counter()
    warnings = changed = 0
    for report in reports:
        found = check(report, rules, principles, targets)
        rows, unread = read_tables(report)
        sha, note = report_commit(report)
        print(f"{shown(report)}: {len(rows)} rows, {len(found)} conflicts, {note}")
        for n, header in unread:
            print(f"  warning: {shown(report)}:{n}: cannot read this change table: {header}")
            warnings += 1
        if not rows:
            print(f"  warning: {shown(report)}: section 2 gives no change row")
            warnings += 1
        for kind, line in found:
            print(f"  {kind}: {line}")
        totals.update(kind for kind, _ in found)
        if sha:
            for line in changed_since(report, sha, now, cache):
                print(f"  changed: {line}")
                changed += 1

    summary = ", ".join(f"{totals[k]} {k}" for k in KINDS)
    print(f"{len(reports)} reports, {sum(totals.values())} conflicts: {summary}; "
          f"{changed} rows changed since their report's commit; {warnings} warnings")
    return 1 if args.strict and totals else 0


if __name__ == "__main__":
    sys.exit(main())
