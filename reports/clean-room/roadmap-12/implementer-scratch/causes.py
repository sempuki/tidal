#!/usr/bin/env python3
"""Sort the tests of the completion line by cause, and measure the gate.

Usage (from the repository root):
    causes.py RESULTS_TSV R24_TSV R23_TSV

RESULTS_TSV holds the lines of `tidal-fe conform tests -v` whose second
column is a test path (status, test, rules, message), from the build at
the commit read. R24_TSV and R23_TSV are `conformance.tsv` at 53577f27
(round 24) and 1017b19c (round 23).

The cause of each of the 104 tests of round 25 and its rewrite mark come
from the table of front-end report 25, section 7. The three P3 tests that
arrived after round 25 get a cause by the reader's judgment. The cause
groups are the reader's judgment from those causes.
"""
import collections
import pathlib
import re
import sys

PASSING = {"pass", "EDG-ONLY", "EDG-ALLOWED", "pass-partial", "NO-DIAGNOSTIC-ALLOWED"}
REPORT = pathlib.Path("reports/clean-room/rust-frontend/report-25.md")
LEDGER = pathlib.Path("reports/clean-room/rust-frontend/ledger.md")

P3 = {
    "programs/P3/pipeline-first-try.cpp":
        ("NAM-9, INST-5", "A member definition of a base over the derived class is made with the class (P3-7)."),
    "templates/instantiation/member-return-type-waits-for-definition.cpp":
        ("INST-5, INST-7", "The return type of an unused member definition is made with the class (P3-7)."),
    "templates/instantiation/member-constructor-made-at-call.cpp":
        ("INST-5, INST-7", "A constructor body is checked while the derived class is incomplete (P3-8)."),
}

# Cause groups, first match wins. Each predicate takes (test, rules, cause, rewrite).
CTAD = {"programs/roadmap/r10-tuple-second-try.cpp",
        "templates/declarations/constant-parameter-class-template.cpp"}
GROUPS = [
    ("rewrite: current instantiation (model section 2)", lambda t, r, c, w: w and re.search(r"sections? 2\b", w)),
    ("rewrite: dependence marks (model section 1)", lambda t, r, c, w: w and re.search(r"sections? 1\b", w)),
    ("rewrite: specialization, point, and order (model section 3)", lambda t, r, c, w: w and re.search(r"sections? 3\b", w)),
    ("rewrite: immediate context (model section 4)", lambda t, r, c, w: w and re.search(r"sections? 4\b", w)),
    ("rewrite: checks at the definition (model section 5)", lambda t, r, c, w: w and re.search(r"sections? 5\b", w)),
    ("INST-5 order of a member's parts (P3-7, P3-8)", lambda t, r, c, w: t in P3),
    ("class template argument deduction", lambda t, r, c, w:
        t.startswith("templates/class-template-deduction/") or t in CTAD),
    ("explicit conversion function to a reference (ICS-45)", lambda t, r, c, w: r.startswith("ICS-45")),
    ("BF-9 layout of wide bit-fields", lambda t, r, c, w: "BF-9" in r),
    ("noexcept operator before the specification is known", lambda t, r, c, w: "noexcept" in c),
    ("constant evaluation of a construct", lambda t, r, c, w:
        "constant" in c.lower() and "unsupported" in STATUS[t]),
    ("template checks outside the model (missed rejects)", lambda t, r, c, w:
        t.startswith("templates/") and STATUS[t] in ("MISSED-REJECT",)),
    ("template constructs outside the model (unsupported)", lambda t, r, c, w:
        t.startswith("templates/") and STATUS[t] == "unsupported"),
    ("EDG-AGREES outside the model", lambda t, r, c, w: STATUS[t] == "EDG-AGREES"),
    ("missed rejects, other chapters", lambda t, r, c, w: STATUS[t] == "MISSED-REJECT"),
    ("unsupported, other chapters", lambda t, r, c, w: STATUS[t] == "unsupported"),
    ("false reject, other", lambda t, r, c, w: STATUS[t] == "FALSE-REJECT"),
]
STATUS: dict[str, str] = {}


def kinds():
    out = {}
    for f in pathlib.Path("specification").rglob("*.md"):
        for m in re.finditer(r":::\{rule\} (\S+)\n:kind: (\S+)", f.read_text()):
            out[m.group(1)] = m.group(2)
    return out


def out_of_scope():
    found = set()
    for line in LEDGER.read_text().splitlines():
        m = re.match(r"^\| G\d+ \| [^|]+ \| out of scope \| ([^|]*)\|", line)
        if m:
            found |= set(re.findall(r"`([^`]+\.cpp)`", m.group(1)))
    return found


def read_tsv(path):
    out = {}
    for line in open(path):
        if line.startswith("#") or not line.strip():
            continue
        parts = line.rstrip("\n").split("\t")
        out[parts[1]] = parts[0]
    return out


def report_table():
    rows = {}
    text = REPORT.read_text().split("## 7.")[1].split("By page:")[0]
    last = ""
    for line in text.splitlines():
        cells = [c.strip() for c in line.strip().strip("|").split("|")]
        if len(cells) == 5 and cells[0].startswith("`"):
            cause = cells[3]
            # "The same." and "As x." rows take the cause of the row above.
            if cause.startswith("The same") or cause.startswith("As "):
                cause = last + " " + cause
            last = cause
            rows[cells[0].strip("`")] = (cells[1], cause, cells[4])
    return rows


def main():
    results, r24, r23 = sys.argv[1:4]
    oos = out_of_scope()
    k = kinds()
    rules_of, msg_of = {}, {}
    for line in open(results):
        p = line.rstrip("\n").split("\t")
        STATUS[p[1]] = p[0]
        rules_of[p[1]] = p[2] if len(p) > 2 else ""
        msg_of[p[1]] = p[3] if len(p) > 3 else ""
    nodiag = {r for r, kind in k.items() if kind == "no-diagnostic"}
    for t, s in list(STATUS.items()):
        if s == "MISSED-REJECT" and rules_of[t] and set(rules_of[t].split()) <= nodiag:
            STATUS[t] = "NO-DIAGNOSTIC-ALLOWED"
    failing = sorted(t for t, s in STATUS.items()
                     if s not in PASSING and not s.startswith("run-") and t not in oos)
    stops = sorted(t for t, s in STATUS.items() if s.startswith("run-") and t not in oos)
    table = report_table()
    print(f"completion line: {len(failing)} tests fail their check (out of scope left out: {len(oos)})")
    print(f"report 25 table rows: {len(table)}; failing tests with no row: "
          f"{sorted(t for t in failing if t not in table)}")
    print(f"rows of the table that now pass: {sorted(t for t in table if t not in failing)}")

    rows = []
    for t in failing:
        if t in table:
            rules, cause, rewrite = table[t]
        else:
            rules, cause = P3[t]
            rewrite = ""
        rows.append((t, rules, cause, rewrite))

    print("\n== by status")
    for s, n in collections.Counter(STATUS[t] for t in failing).most_common():
        print(f"  {s}: {n}")
    print(f"  rewrite marks: {sum(bool(w) for _, _, _, w in rows)}")

    print("\n== by cause group")
    grouped = collections.defaultdict(list)
    for t, r, c, w in rows:
        for name, pred in GROUPS:
            if pred(t, r, c, w):
                grouped[name].append((t, r))
                break
        else:
            grouped["unplaced"].append((t, r))
    for name, _ in GROUPS + [("unplaced", None)]:
        if grouped[name]:
            items = grouped[name]
            print(f"  {name}: {len(items)}")
            for t, r in items:
                print(f"      {STATUS[t]:15} {t}  [{r}]")

    print("\n== by chapter and status")
    tab = collections.defaultdict(collections.Counter)
    for t in failing:
        tab[t.split("/")[0]][STATUS[t]] += 1
    for ch, c in sorted(tab.items(), key=lambda kv: -sum(kv[1].values())):
        print(f"  {ch}: {sum(c.values())} {dict(c)}")

    print("\n== by rule class of the cited rules")
    cls = collections.Counter()
    for t in failing:
        ks = sorted({k.get(r, "?") for r in rules_of[t].split()})
        cls["+".join(ks)] += 1
    for c, n in cls.most_common():
        print(f"  {c}: {n}")

    print("\n== stability against rounds 23 and 24")
    b24, b23 = read_tsv(r24), read_tsv(r23)
    old24 = [t for t in failing if t in b24]
    new24 = [t for t in failing if t not in b24]
    same24 = [t for t in old24 if b24[t] == STATUS[t]]
    print(f"  failing now and present at round 24: {len(old24)}, same status {len(same24)}; "
          f"tests added after round 24: {len(new24)}")
    print(f"  of those, failing at round 23 too: {sum(1 for t in old24 if t in b23 and b23[t] not in PASSING and not b23[t].startswith('run-'))}")
    r24fail = {t for t, s in b24.items() if s not in PASSING and not s.startswith("run-") and t not in oos}
    print(f"  round 24 in-scope failures: {len(r24fail)}; passing now: "
          f"{sum(1 for t in r24fail if t in STATUS and (STATUS[t] in PASSING or STATUS[t].startswith('run-')) or t not in STATUS)}")
    worse = [t for t in b24 if t in STATUS and b24[t] in PASSING and STATUS[t] not in PASSING]
    print(f"  passing at round 24 and not passing now: {worse}")
    rw = [t for t, _, _, w in rows if w]
    print(f"  rewrite tests present at round 24: {sum(t in b24 for t in rw)} of {len(rw)}")

    print("\n== the gate of decision 0062")
    nonrw = len(failing) - len(rw)
    print(f"  in-scope failures {len(failing)}; gate 50; over by {len(failing) - 50}")
    print(f"  without rewrite marks: {nonrw}; to reach 50 with the {len(rw)} marked tests kept, "
          f"the old code passes {len(failing) - 50} of the {nonrw} others")
    print(f"  with the marked tests left out of the count: {nonrw}, over by {nonrw - 50}")

    print("\n== run stops (pass their check)")
    print(f"  {len(stops)}")
    for t in stops:
        print(f"      {t}  [{rules_of[t]}]  {msg_of[t]}")

    programs = sorted(t for t in STATUS if t.startswith("programs/") and STATUS[t] not in PASSING)
    print(f"\n== program tests that do not run to their result: {programs}")


if __name__ == "__main__":
    main()
