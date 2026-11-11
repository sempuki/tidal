"""Count the old front end's failures and join them with the parse count.

Reads conformance.tsv (round 27), the marks file, the front-end ledger's
out-of-scope rows, the round 26 baseline (a91ccc77) through git, and
parse-causes.tsv (from parse_causes.py). The cause groups of the 38
unmarked tests follow report 27 section 8; the grouping is my judgment.

Run from the repository root after parse_causes.py:
    python3 -I reports/clean-room/roadmap-14/implementer-scratch/causes.py
"""
import re
import subprocess
from collections import Counter, defaultdict
from pathlib import Path

ROOT = Path.cwd()
RF = ROOT / "reports/clean-room/rust-frontend"
HERE = ROOT / "reports/clean-room/roadmap-14/implementer-scratch"
PASSING = {"pass", "EDG-ONLY", "EDG-ALLOWED", "pass-partial", "NO-DIAGNOSTIC-ALLOWED"}


def baseline(text: str) -> dict[str, str]:
    out = {}
    for line in text.splitlines():
        if line and not line.startswith("#"):
            s, n = line.split("\t")[:2]
            out[n] = s
    return out


def check_pass(s: str) -> bool:
    return s in PASSING or s.startswith("run-")


now = baseline((RF / "conformance.tsv").read_text())
r26 = baseline(subprocess.run(["git", "show", "a91ccc77:reports/clean-room/rust-frontend/conformance.tsv"],
                              capture_output=True, text=True, check=True).stdout)
marks = {}
for line in (RF / "rewrite-marks.tsv").read_text().splitlines():
    if line and not line.startswith("#"):
        t, sec, _ = line.split("\t")
        marks[t] = sec
oos = set()
for line in (RF / "ledger.md").read_text().splitlines():
    if m := re.match(r"^\| G\d+ \| [^|]+ \| out of scope \| ([^|]*)\|", line):
        oos |= set(re.findall(r"`([^`]+\.cpp)`", m.group(1)))

failing = sorted(n for n, s in now.items() if not check_pass(s) and n not in oos)
unmarked = [n for n in failing if n not in marks]
print(f"failing their check (completion line): {len(failing)}; marked {len(failing) - len(unmarked)}; "
      f"unmarked {len(unmarked)}; out of scope {len(oos & set(now))}")

# The cause groups of the 38, by the rule IDs that report 27 section 8 gives.
GROUPS = {
    "Template checks outside the marks (missed rejects)": [
        "class-argument-reference-temporary", "concept-for-template", "placeholder-parameter-decltype-auto",
        "explicit-instantiation-member-not-satisfied", "alias-template-self-through-class",
        "constant-parameter-no-constexpr-constructor", "nondeduced-only-parameter",
        "explicit-member-class-template-no-prefix"],
    "Template constructs outside the marks (unsupported)": [
        "pointer-subobject", "concept-id-value", "placeholder-parameter.cpp", "function-type-specialization",
        "ellipsis-after-parameter", "expansion-elements"],
    "Class template argument deduction and constant parameters of class template type": [
        "constant-parameter-guide", "sites-for-and-condition", "class-template-deduction/sites.cpp",
        "constant-parameter-class-template", "literal-operator-template-class-parameter",
        "tie-own-constructor-template"],
    "Constant evaluation of a construct (unsupported)": [
        "fold-expansion", "reference-to-local-not-usable", "ordering-defaulted-specified-needs-conversion",
        "init-direct-explicit-conversion"],
    "Constant evaluation and immediate functions (missed rejects)": [
        "base-init-dynamic-cast-early-constant", "read-extended-temporary-not-const", "lambda-escalates-runtime"],
    "Missed rejects in other chapters": [
        "spaceship-member-returns-non-category", "nested-class-member-evaluated",
        "default-template-argument-incomplete-class", "adl-function-template-parameter-adds-nothing",
        "address-no-target-auto-template-parameter"],
    "Unsupported constructs in other chapters": [
        "capture-default-in-member-initializer", "capture-visible-in-trailing-return",
        "overload-set-argument-sequence"],
    "EDG-AGREES": ["trivially-copyable-member-template-selected", "friend-distinct-from-namespace"],
    "False reject": ["tuple-member-get-type-parameter-uses-free"],
}
where = {}
for g, keys in GROUPS.items():
    for k in keys:
        hits = [n for n in unmarked if k in n]
        assert len(hits) == 1, (k, hits)
        where[hits[0]] = g
assert len(where) == len(unmarked), set(unmarked) - set(where)

print("\n| Cause | Tests | Statuses |")
print("|---|---|---|")
for g in GROUPS:
    ts = [n for n in unmarked if where[n] == g]
    st = Counter(now[n] for n in ts)
    print(f"| {g} | {len(ts)} | {', '.join(f'{v} {k}' for k, v in sorted(st.items()))} |")

print("\nMarked, by model section and status:")
by = defaultdict(Counter)
for n in failing:
    if n in marks:
        by[marks[n]][now[n]] += 1
for sec in sorted(by):
    print(f"  section {sec}: {sum(by[sec].values())}: {dict(by[sec])}")

print("\nBy chapter (failing):")
ch = defaultdict(lambda: [0, 0])
for n in failing:
    ch[n.split("/")[0]][0 if n in marks else 1] += 1
for c, (m, u) in sorted(ch.items(), key=lambda kv: -sum(kv[1])):
    print(f"  {c}: marked {m}, unmarked {u}")

# Against round 26.
fail26 = {n for n, s in r26.items() if not check_pass(s)}
print(f"\nunmarked now that failed at round 26: {sum(n in fail26 for n in unmarked)}; "
      f"new since round 26: {sum(n not in r26 for n in unmarked)}")
print(f"failing now that passed their check at round 26: "
      f"{sorted(n for n in failing if n in r26 and check_pass(r26[n]))}")
passed27 = sorted(n for n in r26 if n in fail26 and n in now and check_pass(now[n]))
print(f"round 26 failures that pass their check now: {len(passed27)}")
runstops = sorted(n for n, s in now.items() if s == "run-unsupported")
print(f"run stops: {len(runstops)}")

# Join with the parse count.
pc = {}
for line in (HERE / "parse-causes.tsv").read_text().splitlines():
    t, kind, group, *_ = line.split("\t")
    pc[t] = (kind, group)
print(f"\nmarked tests that the new parser does not parse as needed: "
      f"{[(n, pc[n][1]) for n in marks if n in pc]}")
print(f"unmarked failing tests with a parse disagreement: {[(n, pc[n][1]) for n in unmarked if n in pc]}")
old_pass_dis = [t for t in pc if check_pass(now.get(t, ""))]
print(f"parse disagreements on tests that the old front end passes its check on: {len(old_pass_dis)} of {len(pc)}")
bych = defaultdict(Counter)
for t, (kind, group) in pc.items():
    side = "reject" if kind == "reject" else "tree"
    bych[t.split("/")[0]][side] += 1
print("parse disagreements by chapter (tree side, reject side):")
for c in sorted(bych, key=lambda c: -sum(bych[c].values())):
    print(f"  {c}: {bych[c]['tree']}, {bych[c]['reject']}")
gtree = Counter(g for t, (k, g) in pc.items() if k != "reject" and g.startswith("G"))
print(f"tree-side disagreements in a G group (by pattern): {sum(gtree.values())}")
# Judged tests by side, from the test headers.
sides = Counter()
for p in sorted((ROOT / "tests").rglob("*.cpp")):
    if p.name.endswith(".tu.cpp"):
        continue
    for ln in p.read_text(errors="replace").splitlines():
        if not ln.startswith("//"):
            break
        if ln.startswith("// tidal-expect:"):
            sides[ln.split(":", 1)[1].split()[0]] += 1
            break
print(f"test files by base expectation: {dict(sides)}")
