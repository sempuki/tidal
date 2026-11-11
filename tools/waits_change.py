#!/usr/bin/env python3
"""Give the change of the open questions by **Waits for:** item between two commits.

Usage: tools/waits_change.py START END

A phase close copies these lines (decision 0064). An open question is
one whose Status is "provisional". Each item of a question's Waits for
line counts once. A question counts once more under "names pending.md
row N" for each row N that any of its items names, so a row count covers
every question whose line names the row, whatever the item's wording.
"""
import re
import subprocess
import sys
from collections import Counter

ROW = re.compile(r"\brows? ((?:\d+[a-z]?(?:, | and | or |, and )?)+)")


def show(rev: str, path: str) -> str:
    return subprocess.run(["git", "show", f"{rev}:{path}"], capture_output=True, text=True, check=True).stdout


def counts(rev: str) -> tuple[int, Counter]:
    names = subprocess.run(["git", "ls-tree", "--name-only", rev, "questions/"],
                           capture_output=True, text=True, check=True).stdout.split()
    items: Counter = Counter()
    total = 0
    for name in names:
        if not re.match(r"questions/Q-\d{4}-", name):
            continue
        text = show(rev, name)
        status = re.search(r"^\*\*Status:\*\* *(.+?)\s*$", text, re.M)
        if not status or status.group(1) != "provisional":
            continue
        total += 1
        waits = re.search(r"^\*\*Waits for:\*\* *(.+?)\s*$", text, re.M)
        line = waits.group(1) if waits else "none"
        for item in (part.strip() for part in line.split(";")):
            items[item] += 1
        rows = set()
        for m in ROW.finditer(line):
            rows.update(re.findall(r"\d+[a-z]?", m.group(1)))
        for row in rows:
            items[f"names pending.md row {row}"] += 1
    return total, items


def main() -> int:
    if len(sys.argv) != 3:
        print(__doc__.strip().splitlines()[2], file=sys.stderr)
        return 2
    start, end = sys.argv[1:]
    a_total, a = counts(start)
    b_total, b = counts(end)
    print(f"open questions: {a_total} to {b_total}")
    for item in sorted(set(a) | set(b)):
        if a[item] != b[item]:
            print(f"{item}: {a[item]} to {b[item]}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
