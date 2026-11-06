#!/usr/bin/env python3
"""Counts for the round 9 implementer report.

Usage: counts.py CONFORM_TSV OLD_BASELINE

CONFORM_TSV holds the rows of `tidal-fe conform tests -v` (status, test,
rules, first error), as failures.tsv beside this script does.
OLD_BASELINE is conformance.tsv at the commit round 8 read.
Run from the repository root.
"""
import collections
import pathlib
import re
import sys

PASSING = {"pass", "EDG-ONLY", "EDG-ALLOWED", "pass-partial"}


def rules_by_kind():
    rules = {}
    for f in pathlib.Path("specification").rglob("*.md"):
        for m in re.finditer(r":::\{rule\} (\S+)\n:kind: (\S+)", f.read_text()):
            rules[m.group(1)] = m.group(2)
    return rules


def main():
    now = {}
    for line in open(sys.argv[1]):
        p = line.rstrip("\n").split("\t")
        now[p[1]] = p[0]
    old = {}
    for line in open(sys.argv[2]):
        if line.startswith("#") or not line.strip():
            continue
        s, n = line.rstrip("\n").split("\t")[:2]
        old[n] = s
    fail = {n: s for n, s in now.items() if s not in PASSING}
    print("failing:", len(fail))
    print("new failing:", sorted(n for n in fail if n not in old))
    print("changed status:", sorted(n for n in fail if n in old and old[n] != fail[n]))
    print("fixed:", sorted(n for n, s in old.items() if s not in PASSING
                           and now.get(n, "pass") in PASSING))

    # Failure rate by test directory.
    tot, fl = collections.Counter(), collections.Counter()
    for f in pathlib.Path("tests").rglob("*.cpp"):
        n = f.relative_to("tests").as_posix()
        if n.endswith(".tu.cpp"):
            continue
        d = "/".join(n.split("/")[:2])
        tot[d] += 1
        if now.get(n, "pass") not in PASSING:
            fl[d] += 1
    for r, d in sorted(((fl[d] / tot[d], d) for d in tot if fl[d]), reverse=True)[:12]:
        print(f"{d}: {fl[d]} of {tot[d]} ({100 * r:.1f}%)")

    # Rules named in the front end's source, by kind.
    rules = rules_by_kind()
    src = "".join(p.read_text() for p in pathlib.Path("frontend/src").rglob("*.rs"))
    for kind in ("reject", "undefined", "unspecified", "impl-defined"):
        ids = [r for r, k in rules.items() if k == kind]
        named = [r for r in ids if re.search(r"\b" + re.escape(r) + r"\b", src)]
        print(f"{kind}: {len(named)} of {len(ids)} named in frontend/src")

    # Rules whose run tests (outside programs/) all fail on the front end.
    run = collections.defaultdict(list)
    for f in pathlib.Path("tests").rglob("*.cpp"):
        n = f.relative_to("tests").as_posix()
        if n.endswith(".tu.cpp") or n.startswith("programs/"):
            continue
        head = "\n".join(l for l in f.read_text().splitlines()[:20] if l.startswith("//"))
        m = re.search(r"^// tidal-rules:(.*)$", head, re.M)
        e = re.search(r"^// tidal-expect: (\S+)", head, re.M)
        interp = re.search(r"tidal-interpreter:\s*run-fail", head)
        if m and ((e and e.group(1) in ("run", "run-fail")) or interp):
            for r in m.group(1).split():
                run[r].append(now.get(n, "pass"))
    bad = sorted(r for r in run if not any(s in PASSING for s in run[r]))
    print(f"{len(run)} rules with a run test; none passing for {len(bad)}:")
    for r in bad:
        print(" ", r, rules.get(r), run[r])


if __name__ == "__main__":
    main()
