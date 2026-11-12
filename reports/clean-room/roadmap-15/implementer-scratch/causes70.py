"""Sort the 70 tests that the old front end fails at HEAD, for the round
15 implementer report.

The old front end's status is the baseline conformance.tsv (round 28)
with the statuses of the 17 tests added after it, from
check-next-output.txt ("new test ..." lines; a new test that the tool
does not list passes). A test fails when its status is neither a full
pass nor a run- status, and the front-end ledger does not mark it out of
scope (the rule of the completion line). Marked tests go by model
section. The unmarked tests go to the cause group of CAUSE, which
follows report-28 section 8 and the RV10 repair commit (88d14ea2) by my
judgment, with the step of DESIGN-rewrite.md section 5 that writes the
code again. Prints the tables and writes causes70.tsv. Run from the
repository root.
"""
import re
import sys
from collections import Counter, defaultdict
from pathlib import Path

ROOT = Path.cwd()
HERE = ROOT / "reports/clean-room/roadmap-15/implementer-scratch"
sys.dont_write_bytecode = True
sys.path.insert(0, str(ROOT / "tools"))
import frontend_check as fc  # noqa: E402

# group: (step that writes the code again, [test-path fragments])
CAUSE = {
    "template checks outside the marks (missed rejects)": ("5-7", [
        "class-argument-reference-temporary", "concept-for-template", "placeholder-parameter-decltype-auto",
        "explicit-instantiation-member-not-satisfied", "alias-template-self-through-class",
        "constant-parameter-no-constexpr-constructor", "nondeduced-only-parameter",
        "explicit-member-class-template-no-prefix", "tie-own-constructor-template",
        "two-template-heads-in-class", "using-enum-dependent"]),
    "template constructs outside the marks (unsupported)": ("5-7", [
        "pointer-subobject", "sites-for-and-condition", "concept-id-value", "placeholder-parameter.cpp",
        "function-type-specialization", "ellipsis-after-parameter", "expansion-elements"]),
    "EDG-AGREES in class and constraint code": ("4-5", [
        "trivially-copyable-member-template-selected", "friend-distinct-from-namespace"]),
    "lambdas (closure classes)": ("3 rest / 4", [
        "lambda-escalates-runtime", "capture-visible-in-trailing-return"]),
    "structured bindings": ("3 rest", ["tuple-member-get-type-parameter-uses-free"]),
    "lookup and complete-class checks": ("3 / 4", [
        "nested-class-member-evaluated", "default-template-argument-incomplete-class",
        "adl-function-template-parameter-adds-nothing"]),
    "overload sets as arguments": ("4 / 5", [
        "overload-set-argument-sequence", "address-no-target-auto-template-parameter"]),
    "RV10 grammar forms (decltype(auto), attributes, new_ptr)": ("3 rest", [
        "maybe-unused-on-using-directive", "decltype-auto-destructor-call", "decltype-auto-functional-cast",
        "new-type-pointer-forms"]),
}


def main() -> None:
    base = fc.read_baseline()
    for line in (HERE / "check-next-output.txt").read_text().splitlines():
        if m := re.match(r"^frontend: new test (\S+): (\S+)$", line):
            base[m.group(1)] = m.group(2)
    for p in (ROOT / "tests").rglob("*.cpp"):
        n = p.relative_to(ROOT / "tests").as_posix()
        if not n.endswith(".tu.cpp"):
            base.setdefault(n, "pass")
    oos = fc.out_of_scope()
    marks, _ = fc.rewrite_marks()
    scope = {l.split("\t")[0]: l.split("\t")[1] for l in (HERE / "scopes.tsv").read_text().splitlines()
             if not l.startswith("#")}
    failing = sorted(n for n, s in base.items() if fc.rank(s) == 0 and n not in oos)
    print(f"{len(failing)} tests fail their check; {sum(n in marks for n in failing)} marked")
    print("statuses:", dict(Counter(base[n] for n in failing)))
    rows = []
    by_section = defaultdict(list)
    for n in failing:
        if n in marks:
            by_section[marks[n]].append(n)
            rows.append((n, base[n], f"marked, section {marks[n]}", scope.get(n, "?")))
    print("\nmarked, by model section:")
    for sec in sorted(by_section):
        print(f"  section {sec}: {len(by_section[sec])}  "
              f"{dict(Counter(base[n] for n in by_section[sec]))}")
    unmarked = [n for n in failing if n not in marks]
    groups = defaultdict(list)
    for n in unmarked:
        g = next((g for g, (_, frags) in CAUSE.items() if any(f in n for f in frags)), "no group")
        groups[g].append(n)
        rows.append((n, base[n], g, scope.get(n, "?")))
    print(f"\nunmarked: {len(unmarked)}")
    for g, names in groups.items():
        step = CAUSE.get(g, ("?",))[0]
        print(f"  {len(names)}\t{g}\t(step {step})\t{dict(Counter(base[n] for n in names))}"
              f"\tscopes {dict(Counter(scope.get(n, '?') for n in names))}")
        for n in names:
            print(f"      {base[n]}\t{n}")
    with open(HERE / "causes70.tsv", "w") as f:
        f.write("# test\told status\tcause\tscope (scopes.py)\n")
        for r in rows:
            f.write("\t".join(r) + "\n")
    stops = sorted(n for n, s in base.items() if fc.rank(s) == 1)
    print(f"\nrun stops (pass their check, stop in the run): {len(stops)}")
    print("  by scope:", dict(Counter(scope.get(n, "?") for n in stops)))


if __name__ == "__main__":
    main()
