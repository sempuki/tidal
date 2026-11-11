#!/usr/bin/env python3
"""Count the targets of the parse count that DESIGN-rewrite.md section 5
names: the tests whose outcome needs a tree, the tests that cite TR-12
alone and so stop in the parser, and the reject tests whose stop the
count cannot judge from the header. Also counts the Syntax segments'
productions, the unit of the one-function-per-production check.
Run: python3 -I -B reports/clean-room/roadmap-13/implementer-scratch/parse_targets.py
"""
import re
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]
HERE = Path(__file__).resolve().parent
tests = ROOT / "tests"
kinds = Counter()
tr12_alone, tr12_any, multi_tu = [], 0, 0
for p in sorted(tests.rglob("*.cpp")):
    name = p.relative_to(tests).as_posix()
    if name.endswith(".tu.cpp"):
        multi_tu += 1
        continue
    head = []
    for line in p.read_text(errors="replace").splitlines():
        if not line.startswith("//"):
            break
        head.append(line)
    rules = set()
    expect = None
    for h in head:
        if h.startswith("// tidal-rules:"):
            rules |= set(h.split(":", 1)[1].split())
        if h.startswith("// tidal-expect:") and expect is None:
            expect = h.split(":", 1)[1].split()[0] if h.split(":", 1)[1].split() else ""
    if expect is None:
        kinds["no base expectation"] += 1
        continue
    if "TR-12" in rules:
        tr12_any += 1
    if expect == "reject" and rules == {"TR-12"}:
        tr12_alone.append(name)
        kinds["reject, TR-12 alone (stops in the parser)"] += 1
    elif expect == "reject":
        kinds["reject, other rules (not judged)"] += 1
    else:
        kinds[f"{expect} (needs a tree)"] += 1

out = [f"test files (not .tu.cpp): {sum(kinds.values())}; extra translation units: {multi_tu}"]
out += [f"  {v:5d}  {k}" for k, v in kinds.most_common()]
out.append(f"tests that cite TR-12 at all: {tr12_any}")
spec = ROOT / "specification"
prods, pages = Counter(), set()
for f in spec.rglob("*.md"):
    for block in re.findall(r"\.\. productionlist:: tidal\n(.*?)```", f.read_text(), re.S):
        for m in re.finditer(r"^\s+([a-z_][a-z0-9_]*):", block, re.M):
            prods[m.group(1)] += 1
            pages.add(f.relative_to(spec).as_posix())
out.append(f"grammar nonterminals in the Syntax segments: {len(prods)}, on {len(pages)} pages")
(HERE / "parse-targets-output.txt").write_text("\n".join(out) + "\n")
print("\n".join(out))
