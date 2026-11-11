#!/usr/bin/env python3
"""For each test of the completion line, the questions its header or
notes name, the status line of each, and its Waits for line.

Usage (from the repository root): questions.py CAUSES_OUTPUT
CAUSES_OUTPUT is the output of causes.py; the test paths are read from
its cause-group section.
"""
import collections
import pathlib
import re
import sys


def main():
    tests = []
    for line in open(sys.argv[1]):
        m = re.match(r"^\s{6}\S+\s+(\S+\.cpp)\s+\[", line)
        if m and "run stops" not in line:
            tests.append(m.group(1))
    # The run-stop section also matches; keep the first 107 (cause groups).
    seen, ordered = set(), []
    for t in tests:
        if t not in seen:
            seen.add(t)
            ordered.append(t)
    qdir = pathlib.Path("questions")
    status, waits = {}, {}
    for f in qdir.glob("Q-*.md"):
        text = f.read_text()
        q = f.name[:6]
        m = re.search(r"^\*\*Status:\*\*\s*(.+)$", text, re.M)
        status[q] = m.group(1).strip() if m else "?"
        m = re.search(r"^\*\*Waits for:\*\*\s*(.+)$", text, re.M)
        waits[q] = m.group(1).strip() if m else ""
    by_wait = collections.Counter()
    for t in ordered:
        path = pathlib.Path("tests") / t
        if not path.exists():
            continue
        qs = sorted(set(re.findall(r"Q-\d{4}", path.read_text())))
        open_qs = [q for q in qs if q in status and status[q].lower().startswith("provisional")]
        if open_qs:
            print(f"{t}: " + "; ".join(f"{q} ({status[q][:40]}; waits for: {waits[q][:60]})" for q in open_qs))
            for q in open_qs:
                by_wait[waits[q][:60]] += 1
    print(f"\n{len(ordered)} tests read")
    for w, n in by_wait.most_common():
        print(f"  {n}: {w}")


if __name__ == "__main__":
    main()
