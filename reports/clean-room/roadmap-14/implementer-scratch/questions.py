"""List the questions that the headers of the 74 tests of the completion line
name, with each question's status and Waits for line, and whether the test is
marked. Also the questions that the G166 to G179 ledger rows and report-rw1.md
name.

Run from the repository root:
    python3 -I reports/clean-room/roadmap-14/implementer-scratch/questions.py
"""
import re
from collections import defaultdict
from pathlib import Path

ROOT = Path.cwd()
RF = ROOT / "reports/clean-room/rust-frontend"
PASSING = {"pass", "EDG-ONLY", "EDG-ALLOWED", "pass-partial", "NO-DIAGNOSTIC-ALLOWED"}
marks = {l.split("\t")[0] for l in (RF / "rewrite-marks.tsv").read_text().splitlines() if l and not l.startswith("#")}
oos = set()
for line in (RF / "ledger.md").read_text().splitlines():
    if m := re.match(r"^\| G\d+ \| [^|]+ \| out of scope \| ([^|]*)\|", line):
        oos |= set(re.findall(r"`([^`]+\.cpp)`", m.group(1)))
failing = []
for l in (RF / "conformance.tsv").read_text().splitlines():
    if l and not l.startswith("#"):
        s, n = l.split("\t")
        if s not in PASSING and not s.startswith("run-") and n not in oos:
            failing.append(n)
named = defaultdict(list)
for n in failing:
    head = "\n".join(l for l in (ROOT / "tests" / n).read_text().splitlines() if l.startswith("//"))
    for q in sorted(set(re.findall(r"Q-(\d{4})", head))):
        named[q].append(n)


def info(q: str) -> str:
    f = next((ROOT / "questions").glob(f"Q-{q}-*.md"))
    t = f.read_text()
    status = re.search(r"\*\*Status:\*\*\s*(.*)", t)
    waits = re.search(r"\*\*Waits for:\*\*\s*(.*)", t)
    return f"{status.group(1).strip() if status else '?'}; waits for: {waits.group(1).strip() if waits else '-'}"


print(f"{len(failing)} failing tests; {len(named)} questions named")
for q in sorted(named):
    ts = named[q]
    print(f"Q-{q} ({info(q)}): {len(ts)} tests, {sum(t in marks for t in ts)} marked: {', '.join(ts)}")
g = "\n".join(l for l in (RF / "ledger.md").read_text().splitlines() if re.match(r"^\| G1(6[6-9]|7\d) ", l))
g += (RF / "report-rw1.md").read_text().split("## 10. Findings")[1]
print("\nquestions in the G166 to G179 rows and report-rw1 section 10:")
for q in sorted(set(re.findall(r"Q-(\d{4})", g))):
    print(f"Q-{q} ({info(q)})")
