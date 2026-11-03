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

A report can name the commit it read with a backquoted commit ID in its
header, before the first "## " heading (for example "Commit read:
`3d98c93`."). For such a report the tool also prints how many later
commits change specification/.

The tool exits 0 even when it finds conflicts, because a report is a
record of its date and the check must not fail a commit. --strict makes
it exit 1 when it finds a conflict.

The rule IDs and classes come from parse_spec() in
tools/check_specification.py.

Usage:
    tools/propose_check.py [--strict] [REPORT ...]
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


def shown(path: Path) -> Path:
    return path.relative_to(ROOT) if path.is_relative_to(ROOT) else path


def cells(line: str) -> list[str]:
    return [c.strip() for c in line.strip().strip("|").split("|")]


def change_rows(report: Path):
    """Yield (line number, ID, change, old class) for each row of the section 2 table."""
    inside = False
    header = None
    for n, line in enumerate(report.read_text().splitlines(), 1):
        if CHANGES.match(line):
            inside = True
            continue
        if inside and NEXT.match(line):
            return
        if not inside:
            continue
        if not line.startswith("|"):
            header = None
            continue
        row = cells(line)
        if header is None:
            header = [c.lower() for c in row]
            continue
        if set(line) <= set("|-: "):
            continue
        if "id" not in header or "change" not in header or "old class" not in header:
            continue
        get = dict(zip(header, row))
        yield n, get["id"].strip("`"), get["change"], get["old class"]


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


def commit_note(report: Path) -> str:
    """Say which commit the report read and how far specification/ has moved since."""
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
            return f"read {sha}, {run.stdout.strip()} later commits change specification/"
    return "names no commit"


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


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("reports", nargs="*", type=Path,
                    help="reports to check (default: reports/clean-room/X*/report.md)")
    ap.add_argument("--strict", action="store_true",
                    help="exit 1 when a row conflicts with the specification")
    args = ap.parse_args()

    reports = [p.resolve() for p in args.reports] or sorted(
        REPORTS.glob("X*/report.md"), key=lambda p: int(p.parent.name[1:]))
    rules, _ = parse_spec()
    principles = principle_ids()
    targets = merge_targets()

    totals = Counter()
    for report in reports:
        found = check(report, rules, principles, targets)
        rows = sum(1 for _ in change_rows(report))
        print(f"{shown(report)}: {rows} rows, {len(found)} conflicts, {commit_note(report)}")
        for kind, line in found:
            print(f"  {kind}: {line}")
        totals.update(kind for kind, _ in found)

    summary = ", ".join(f"{totals[k]} {k}" for k in KINDS)
    print(f"{len(reports)} reports, {sum(totals.values())} conflicts: {summary}")
    return 1 if args.strict and totals else 0


if __name__ == "__main__":
    sys.exit(main())
