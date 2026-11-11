#!/usr/bin/env python3
"""For each test whose base expectation is reject, run `tidal-fe check` and
record the rule of the first error. The parse count of DESIGN-rewrite.md
section 5 can use the old front end's TR-12 stops as the target for the
rejects that the header alone cannot judge.
Run after a build of frontend/: python3 -I -B reports/clean-room/roadmap-13/implementer-scratch/first_errors.py
"""
import re
import subprocess
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]
HERE = Path(__file__).resolve().parent
FE = ROOT / "frontend/target/release/tidal-fe"
first = Counter()
tr12 = []
for p in sorted((ROOT / "tests").rglob("*.cpp")):
    if p.name.endswith(".tu.cpp"):
        continue
    head = [l for l in p.read_text(errors="replace").splitlines()[:40] if l.startswith("// tidal-")]
    exp = next((l.split(":", 1)[1].split()[0] for l in head if l.startswith("// tidal-expect:")), None)
    if exp != "reject":
        continue
    r = subprocess.run([str(FE), "check", str(p)], capture_output=True, text=True, timeout=60)
    m = re.search(r"error \[([^\]]+)\]", r.stdout + r.stderr)
    if m:
        rules = [x.strip() for x in m.group(1).split(",")]
        first["rejected, first error " + ("names TR-12" if "TR-12" in rules else "another rule")] += 1
        if "TR-12" in rules:
            tr12.append(p.relative_to(ROOT / "tests").as_posix())
    else:
        first["no error with a rule (a missed reject, a stop, or a test of several units)"] += 1
out = [f"{v:5d}  {k}" for k, v in first.most_common()]
out.append("rejects whose first error on the old front end is TR-12:")
out += [f"       {t}" for t in tr12]
(HERE / "first-errors-output.txt").write_text("\n".join(out) + "\n")
print("\n".join(out[:4]), f"... {len(tr12)} TR-12 first errors")
