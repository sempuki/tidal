#!/usr/bin/env python3
"""Sort the tests of the completion line by cause, check stability, and
measure the rewrite's takeover list.

Reads, from the worktree root:
- implementer-scratch/results.tsv: the rows of `tidal-fe conform tests -v`
  that are not a plain pass, at HEAD (a6a2b16c);
- reports/clean-room/rust-frontend/conformance.tsv (round 26, a91ccc77) and
  the round 25 and round 24 baselines through `git show`;
- the marks file and the out-of-scope rows of the front-end ledger;
- report-26.md section 8: the cause table of the 45 tests.
The cause groups are the implementer's judgment from report 26's causes.
The three P4 tests are added by the implementer's judgment.
Run: python3 -I -B reports/clean-room/roadmap-13/implementer-scratch/causes.py
"""
import re
import subprocess
from collections import Counter, defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]
HERE = Path(__file__).resolve().parent
FE = ROOT / "reports/clean-room/rust-frontend"
PASSING = {"pass", "EDG-ONLY", "EDG-ALLOWED", "pass-partial", "NO-DIAGNOSTIC-ALLOWED"}


def fails_check(s):
    return s not in PASSING and not s.lower().startswith("run-")


def baseline(text):
    out = {}
    for line in text.splitlines():
        if line and not line.startswith("#"):
            s, n = line.split("\t")[:2]
            out[n] = s
    return out


def git_baseline(commit):
    return baseline(subprocess.run(["git", "show", f"{commit}:reports/clean-room/rust-frontend/conformance.tsv"],
                                   cwd=ROOT, capture_output=True, text=True, check=True).stdout)


now = {}
for line in (HERE / "results.tsv").read_text().splitlines():
    c = line.split("\t")
    now[c[1]] = c[0]
rules_of = {line.split("\t")[1]: line.split("\t")[2] for line in (HERE / "results.tsv").read_text().splitlines()}

# NO-DIAGNOSTIC-ALLOWED is applied by frontend_check.py; take it from the baseline.
r26 = baseline((FE / "conformance.tsv").read_text())
for n, s in r26.items():
    if s == "NO-DIAGNOSTIC-ALLOWED" and now.get(n) == "MISSED-REJECT":
        now[n] = s
r25 = git_baseline("b035efd4")
r24 = git_baseline("53577f27")

oos = set()
for line in (FE / "ledger.md").read_text().splitlines():
    if m := re.match(r"^\| G\d+ \| [^|]+ \| out of scope \| ([^|]*)\|", line):
        oos |= set(re.findall(r"`([^`]+\.cpp)`", m.group(1)))

marks = {}
for line in (FE / "rewrite-marks.tsv").read_text().splitlines():
    if line and not line.startswith("#"):
        t, sec, cause = line.split("\t")
        marks[t] = sec

failing = sorted(n for n, s in now.items() if fails_check(s) and n not in oos)
unmarked = [n for n in failing if n not in marks]

# Report 26 section 8 table.
rep = (FE / "report-26.md").read_text()
sec8 = rep.split("### The 45 tests that count toward the gate")[1].split("By page:")[0]
cause26 = {}
for line in sec8.splitlines():
    m = re.match(r"^\| `([^`]+\.cpp)` \| ([^|]+) \| ([^|]+) \| (.+) \|$", line)
    if m:
        cause26[m.group(1)] = (m.group(2).strip(), m.group(3).strip(), m.group(4).strip())

P4 = {
    "declarations/functions/definitions/member-in-class-specialization-over-class.cpp": "P4-1",
    "declarations/functions/definitions/member-in-class-type-completed-after-class.cpp": "P4-1",
    "programs/P4/money-first-try.cpp": "P4-1",
}

GROUPS = [
    ("P4-1: the types of a member defined in its class, checked at the end of the class (FN-12)",
     lambda n, r: n in P4),
    ("Class template argument deduction (CTAD-1, CTAD-19, CTAD-29)",
     lambda n, r: n.startswith("templates/class-template-deduction/") and "constant-parameter" not in n
     or n == "programs/roadmap/r10-tuple-second-try.cpp"),
    ("A constant parameter whose type is a class template name (TPL-11)",
     lambda n, r: n in {"templates/class-template-deduction/constant-parameter-guide.cpp",
                        "templates/declarations/constant-parameter-class-template.cpp",
                        "classes/operators/literal-operator-template-class-parameter.cpp"}),
    ("Template checks outside the marks (missed rejects)",
     lambda n, r: n.startswith("templates/") and now[n] == "MISSED-REJECT"),
    ("Template constructs outside the marks (unsupported)",
     lambda n, r: n.startswith("templates/") and now[n] == "unsupported"),
    ("Constant evaluation of a construct (unsupported)",
     lambda n, r: now[n] == "unsupported" and any(k in r for k in ("PRIM-27", "CE-3", "LSUP-13", "OVL-40"))),
    ("Missed rejects in other chapters",
     lambda n, r: now[n] == "MISSED-REJECT"),
    ("Unsupported constructs in other chapters",
     lambda n, r: now[n] == "unsupported"),
    ("EDG-AGREES", lambda n, r: now[n] == "EDG-AGREES"),
    ("False reject", lambda n, r: now[n] == "FALSE-REJECT"),
]

group_of = {}
for n in unmarked:
    r = rules_of.get(n, "")
    for g, test in GROUPS:
        if test(n, r):
            group_of[n] = g
            break

out = []
p = out.append
p(f"tests that fail their check, in scope: {len(failing)}; marked {len(failing) - len(unmarked)}; unmarked {len(unmarked)}")
p(f"out of scope: {len(oos & set(now))} (of {len(oos)} named)")
p("status of the unmarked: " + ", ".join(f"{k} {v}" for k, v in Counter(now[n] for n in unmarked).most_common()))
p("status of the marked: " + ", ".join(f"{k} {v}" for k, v in Counter(now[n] for n in failing if n in marks).most_common()))
p("marked by section: " + ", ".join(f"{k}: {v}" for k, v in sorted(Counter(marks.values()).items())))
missing = [n for n in unmarked if n not in cause26 and n not in P4]
p(f"unmarked tests with no cause in report 26 or P4: {len(missing)} {missing}")
p("")
p("Cause groups of the unmarked tests:")
by = defaultdict(list)
for n in unmarked:
    by[group_of.get(n, "UNGROUPED")].append(n)
for g, _ in GROUPS + [("UNGROUPED", None)]:
    if by.get(g):
        p(f"  {len(by[g]):2d}  {g}")
        for n in by[g]:
            p(f"        {now[n]:14s} {rules_of.get(n, ''):24s} {n}")
p("")
# Stability.
for label, base in (("round 25 (b035efd4)", r25), ("round 24 (53577f27)", r24)):
    same = sum(1 for n in unmarked if n in base and base[n] == now[n])
    absent = sum(1 for n in unmarked if n not in base)
    worse = sorted(n for n, s in now.items() if n in base and base[n] in PASSING | {x for x in base.values() if x.lower().startswith("run-")} and fails_check(s))
    p(f"unmarked against {label}: {same} same status, {absent} not in that baseline; tests that passed their check there and fail now: {len(worse)}")
new_since_26 = sorted(n for n in now if n not in r26)
p(f"tests with no row in the round 26 baseline that do not pass: {[n for n in new_since_26 if now[n] not in PASSING]}")
p("")
# Run stops.
runs = sorted(n for n, s in now.items() if s.lower().startswith("run-") and n not in oos)
p(f"run stops (pass their check): {len(runs)}")
for n in runs:
    p(f"        {now[n]:16s} {rules_of.get(n, ''):24s} {n}")
progs = sorted(n for n in now if n.startswith("programs/") and now[n] not in PASSING)
p(f"program tests that do not run to their result: {len(progs)} {progs}")
p("")
# Rules changed after RV8 read the tree, and the failing tests that cite them.
changed = "DTOR-8 OPR-P3 OPR-8 OPR-16 OPR-10 UNI-P3 FN-12 REF-2 TYN-10 TYN-6 ADD-11 ADD-14 STG-16 STG-28 CTAD-18 DEP-P1 DEP-40 INST-1 INST-2 INST-5".split()
hits = [(n, sorted(set(rules_of.get(n, "").split()) & set(changed))) for n in failing]
hits = [(n, h) for n, h in hits if h]
p(f"failing tests that cite a rule changed after RV8: {len(hits)}")
for n, h in hits:
    p(f"        {' '.join(h):20s} {'marked ' + marks[n] if n in marks else 'unmarked':10s} {n}")
p("")
# Takeover first list, by section.
for sec in sorted(set(marks.values())):
    p(f"model section {sec}: {sum(1 for v in marks.values() if v == sec)} tests")
    for t, v in marks.items():
        if v == sec:
            p(f"        {now.get(t, 'pass?'):14s} {t}")
(HERE / "causes-output.txt").write_text("\n".join(out) + "\n")
print("\n".join(out))

# Chapter by status, for all 80 and for the 48.
for label, group in (("all", failing), ("unmarked", unmarked)):
    table = defaultdict(Counter)
    for n in group:
        table[n.split("/")[0]][now[n]] += 1
    cols = ["MISSED-REJECT", "unsupported", "FALSE-REJECT", "EDG-AGREES"]
    extra = [f"chapter by status ({label}): " + " | ".join(cols)]
    for ch in sorted(table, key=lambda c: -sum(table[c].values())):
        extra.append(f"  {ch:14s} " + " ".join(f"{table[ch][c]:3d}" for c in cols) + f"  = {sum(table[ch].values())}")
    print("\n".join(extra))
    with open(HERE / "causes-output.txt", "a") as f:
        f.write("\n".join(extra) + "\n")
