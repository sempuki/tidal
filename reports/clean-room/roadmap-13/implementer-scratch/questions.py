#!/usr/bin/env python3
"""List the open questions that the headers of the 80 tests of the
completion line name, with each question's Waits for line, and say whether
the test is marked.
Run after causes.py: python3 -I -B reports/clean-room/roadmap-13/implementer-scratch/questions.py
"""
import re
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]
HERE = Path(__file__).resolve().parent
PASSING = {"pass", "EDG-ONLY", "EDG-ALLOWED", "pass-partial", "NO-DIAGNOSTIC-ALLOWED"}
marks = {l.split("\t")[0] for l in (ROOT / "reports/clean-room/rust-frontend/rewrite-marks.tsv").read_text().splitlines()
         if l and not l.startswith("#")}
oos = set()
for line in (ROOT / "reports/clean-room/rust-frontend/ledger.md").read_text().splitlines():
    if m := re.match(r"^\| G\d+ \| [^|]+ \| out of scope \| ([^|]*)\|", line):
        oos |= set(re.findall(r"`([^`]+\.cpp)`", m.group(1)))
base = {}
for l in (ROOT / "reports/clean-room/rust-frontend/conformance.tsv").read_text().splitlines():
    if l and not l.startswith("#"):
        s, n = l.split("\t")[:2]
        base[n] = s
failing = []
for l in (HERE / "results.tsv").read_text().splitlines():
    s, n = l.split("\t")[:2]
    if base.get(n) == "NO-DIAGNOSTIC-ALLOWED":
        s = base[n]
    if s not in PASSING and not s.lower().startswith("run-") and n not in oos:
        failing.append(n)
named = defaultdict(list)
for n in failing:
    head = "\n".join(l for l in (ROOT / "tests" / n).read_text().splitlines() if l.startswith("//"))
    for q in sorted(set(re.findall(r"Q-(\d{4})", head))):
        named[q].append(n)
out = []
for q in sorted(named):
    f = next((ROOT / "questions").glob(f"Q-{q}-*.md"))
    t = f.read_text()
    st = re.search(r"^\*\*Status:\*\* (\S+)", t, re.M)
    w = re.search(r"^\*\*Waits for:\*\* (.+)$", t, re.M)
    out.append(f"Q-{q} ({st.group(1) if st else '?'}; waits for: {w.group(1) if w else '-'})")
    for n in named[q]:
        out.append(f"        {'marked' if n in marks else 'unmarked'}  {n}")
(HERE / "questions-output.txt").write_text("\n".join(out) + "\n")
print("\n".join(out))
