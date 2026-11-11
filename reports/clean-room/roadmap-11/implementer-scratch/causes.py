#!/usr/bin/env python3
"""Sort the 127 tests of the completion line by cause.

Usage: causes.py FAILURES_TSV
Run from the repository root. The cause of a missed reject is the
reader's judgment from the test's rules, its page, and front-end report
24 sections 3 to 6, since the front end prints nothing for a test that
it accepts. Each test lands in the first group whose test matches.
"""
import collections
import pathlib
import re
import sys

PASS_CHECK = {"pass", "EDG-ONLY", "EDG-ALLOWED", "pass-partial", "run-unsupported"}

# Tests whose cause report 23 section 3 or report 24 section 6 puts in the
# front end's model of a specialization or of the current instantiation.
MODEL = {
    "templates/dependent-names/member-template-current-instantiation-pointer.cpp",
    "templates/dependent-names/member-template-non-dependent-base.cpp",
    "templates/dependent-names/member-template-through-current-instantiation-object.cpp",
    "templates/dependent-names/member-template-current-instantiation-qualifier-dependent-base.cpp",
    "templates/dependent-names/other-specialization-constant-argument-member-missing.cpp",
    "templates/dependent-names/other-specialization-constant-argument-no-typename.cpp",
    "templates/dependent-names/current-instantiation-enclosing-base.cpp",
    "templates/deduction/substitution-member-wrong-kind.cpp",
    "templates/deduction/substitution-lambda-body.cpp",
    "templates/deduction/substitution-lambda-default.cpp",
    "templates/constraints/out-of-class-other-clause.cpp",
    "templates/constraints/out-of-class-without-clause.cpp",
    "templates/packs/fold-in-outer-pattern.cpp",
    "templates/packs/size-operator-in-outer-pattern.cpp",
    "statements/if/constexpr-template-nondependent-call-checked.cpp",
    "templates/specialization/partial-no-primary-variable.cpp",
}
SIZE_TYPE = {
    "declarations/declarators/array-4gib-ilp32.cpp",
    "foundations/storage/array-size-exceeds-size-type.cpp",
}
GROUPS = [
    ("template model (current instantiation, specialization)", lambda n, s, r, m: n in MODEL),
    ("program test, cause open (TR-13 nesting)", lambda n, s, r, m: n == "programs/roadmap/r10-list-first-try.cpp"),
    ("class template argument deduction", lambda n, s, r, m: n.startswith("templates/class-template-deduction/")
        or "deduction of class template arguments" in m or "TPL-11" in r),
    ("EDG-AGREES", lambda n, s, r, m: s == "EDG-AGREES"),
    ("recommended warning", lambda n, s, r, m: s == "MISSED-REJECT" and n.endswith("-warns.cpp")
        or n.endswith("-warns-default.cpp")),
    ("size type width", lambda n, s, r, m: n in SIZE_TYPE),
    ("unsupported: concept in a static_assert", lambda n, s, r, m: s == "unsupported" and "static_assert: concept" in m),
    ("unsupported: BF-9 layout", lambda n, s, r, m: s == "unsupported" and "BF-9" in r),
    ("unsupported: noexcept operator", lambda n, s, r, m: s == "unsupported" and "noexcept operator" in m),
    ("unsupported: new expression in a constant", lambda n, s, r, m: s == "unsupported" and "new expression" in m),
    ("unsupported: other, one or two each", lambda n, s, r, m: s == "unsupported"),
    ("false reject, other", lambda n, s, r, m: s == "FALSE-REJECT"),
    ("missed reject, templates chapter", lambda n, s, r, m: s == "MISSED-REJECT" and n.startswith("templates/")),
    ("missed reject, other chapters", lambda n, s, r, m: s == "MISSED-REJECT"),
]


def out_of_scope():
    found = set()
    for line in open("reports/clean-room/rust-frontend/ledger.md"):
        m = re.match(r"^\| G\d+ \| [^|]+ \| out of scope \| ([^|]*)\|", line)
        if m:
            found |= set(re.findall(r"`([^`]+\.cpp)`", m.group(1)))
    return found


def kinds():
    out = {}
    for f in pathlib.Path("specification").rglob("*.md"):
        for m in re.finditer(r":::\{rule\} (\S+)\n:kind: (\S+)", f.read_text()):
            out[m.group(1)] = m.group(2)
    return out


def main():
    k = kinds()
    oos = out_of_scope()
    groups = collections.defaultdict(list)
    for line in open(sys.argv[1]):
        p = line.rstrip("\n").split("\t") + ["", ""]
        s, n, r, m = p[0], p[1], p[2], p[3]
        if s in PASS_CHECK or n in oos:
            continue
        rules = r.split()
        if s == "MISSED-REJECT" and rules and all(k.get(x) == "no-diagnostic" for x in rules):
            continue  # NO-DIAGNOSTIC-ALLOWED in frontend_check.py
        for name, test in GROUPS:
            if test(n, s, r, m):
                groups[name].append((n, r))
                break
    total = 0
    for name, _ in GROUPS:
        tests = groups.get(name, [])
        total += len(tests)
        print(f"{len(tests):3}  {name}")
        for n, r in tests:
            print(f"       {n}  [{r}]")
    print(f"{total:3}  total")


if __name__ == "__main__":
    main()
