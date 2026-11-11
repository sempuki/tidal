#!/usr/bin/env python3
"""Counts for the round 11 implementer report.

Usage: counts.py FAILURES_TSV OLD_BASELINE [OLDER_BASELINE]

FAILURES_TSV holds the failing rows of `tidal-fe conform tests -v`
(status, test, rules, first error or stop), as failures.tsv beside this
script does. OLD_BASELINE is conformance.tsv after front-end round 23
(1017b19c). OLDER_BASELINE, if given, is conformance.tsv after round 22
(a15a877d). Run from the repository root.
"""
import collections
import pathlib
import re
import sys

PASSING = {"pass", "EDG-ONLY", "EDG-ALLOWED", "pass-partial", "NO-DIAGNOSTIC-ALLOWED"}
CHECK_PASS = PASSING | {"run-unsupported"}


def rules_by_kind():
    rules = {}
    for f in pathlib.Path("specification").rglob("*.md"):
        for m in re.finditer(r":::\{rule\} (\S+)\n:kind: (\S+)", f.read_text()):
            rules[m.group(1)] = m.group(2)
    return rules


def read_baseline(path):
    base = {}
    for line in open(path):
        if line.startswith("#") or not line.strip():
            continue
        s, n = line.rstrip("\n").split("\t")[:2]
        base[n] = s
    return base


def out_of_scope():
    found = set()
    for line in open("reports/clean-room/rust-frontend/ledger.md"):
        m = re.match(r"^\| G\d+ \| [^|]+ \| out of scope \| ([^|]*)\|", line)
        if m:
            found |= set(re.findall(r"`([^`]+\.cpp)`", m.group(1)))
    return found


def cited(name):
    rules = set()
    for line in (pathlib.Path("tests") / name).read_text().splitlines():
        if not line.startswith("//"):
            break
        if line.startswith("// tidal-rules:"):
            rules |= set(line.split(":", 1)[1].split())
    return rules


def message_group(msg):
    """The front end's stop message, with names and line numbers removed."""
    msg = re.sub(r"^\d+:-? ?", "", msg)
    msg = re.sub(r"^unsupported at line \d+: ", "", msg)
    msg = re.sub(r"'[^']*'", "'_'", msg)
    msg = re.sub(r"^check: ", "", msg)
    return msg.strip()


def main():
    kinds = rules_by_kind()
    oos = out_of_scope()
    rows = {}
    for line in open(sys.argv[1]):
        p = line.rstrip("\n").split("\t")
        rows[p[1]] = (p[0], p[2].split() if len(p) > 2 else [], p[3] if len(p) > 3 else "")
    nodiag = {n for n, (s, r, _) in rows.items()
              if s == "MISSED-REJECT" and cited(n) and all(kinds.get(x) == "no-diagnostic" for x in cited(n))}
    status = {n: ("NO-DIAGNOSTIC-ALLOWED" if n in nodiag else s) for n, (s, _, _) in rows.items()}
    fail_check = {n: s for n, s in status.items() if s not in CHECK_PASS}
    completion = {n: s for n, s in fail_check.items() if n not in oos}
    run_stop = {n for n, s in status.items() if s == "run-unsupported"}
    print("rows:", len(rows), "no-diagnostic allowed:", len(nodiag))
    print("fail check:", len(fail_check), "of which out of scope:", len(fail_check) - len(completion))
    print("completion count:", len(completion))
    print("  by status:", dict(collections.Counter(completion.values())))
    print("run-unsupported:", len(run_stop))

    # By chapter and status.
    print("\nby chapter (completion set):")
    ch = collections.defaultdict(collections.Counter)
    for n, s in completion.items():
        ch[n.split("/")[0]][s] += 1
    for c in sorted(ch, key=lambda c: -sum(ch[c].values())):
        print(f"  {c}: {dict(ch[c])} = {sum(ch[c].values())}")
    rc = collections.Counter(n.split("/")[0] for n in run_stop)
    print("run stops by chapter:", dict(rc))

    # By the class of the cited rules.
    print("\nby rule class of the cited rules (completion set):")
    cls = collections.Counter()
    for n, s in completion.items():
        ks = {kinds.get(r, "?") for r in cited(n)}
        if "reject" in ks:
            key = "reject"
        elif "recommended" in ks:
            key = "recommended"
        elif ks <= {"required"}:
            key = "required only"
        elif "no-diagnostic" in ks:
            key = "no-diagnostic with other"
        else:
            key = "+".join(sorted(ks))
        cls[(key, s)] += 1
    for k, v in sorted(cls.items()):
        print(f"  {k}: {v}")

    # Unsupported messages, check side and run side.
    print("\ncheck-side unsupported messages (completion set):")
    msgs = collections.Counter(message_group(rows[n][2]) for n, s in completion.items() if s == "unsupported")
    for m, c in msgs.most_common():
        print(f"  {c:3}  {m}")
    print("\nrun stops:")
    msgs = collections.Counter(message_group(rows[n][2]) for n in run_stop)
    for m, c in msgs.most_common():
        print(f"  {c:3}  {m}")

    # False rejects and EDG-AGREES.
    print("\nfalse rejects and EDG-AGREES (completion set):")
    for n, s in sorted(completion.items()):
        if s in ("FALSE-REJECT", "EDG-AGREES"):
            print(f"  {s} {n} {' '.join(rows[n][1])} :: {rows[n][2][:100]}")

    # Missed rejects by page and rule.
    print("\nmissed rejects by page (completion set):")
    pg = collections.defaultdict(list)
    for n, s in completion.items():
        if s == "MISSED-REJECT":
            pg["/".join(n.split("/")[:2])].append(" ".join(rows[n][1]))
    for p in sorted(pg, key=lambda p: (-len(pg[p]), p)):
        print(f"  {p} ({len(pg[p])}): {'; '.join(pg[p])}")

    # Stability against round 23 and round 22.
    for label, path in (("round 23", sys.argv[2]), ("round 22", sys.argv[3] if len(sys.argv) > 3 else None)):
        if not path:
            continue
        old = read_baseline(path)
        same = [n for n in completion if old.get(n) == completion[n]]
        other = [n for n in completion if n in old and old[n] != completion[n]]
        new = [n for n in completion if n not in old]
        print(f"\nagainst {label}: same status {len(same)}, other status {len(other)}, not in baseline {len(new)}")
        for n in sorted(other):
            print(f"  other: {old[n]} -> {completion[n]} {n}")
        old_in = [n for n, s in old.items() if s not in CHECK_PASS and n not in oos and n not in nodiag
                  and not (s == "MISSED-REJECT" and cited(n) and all(kinds.get(x) == "no-diagnostic" for x in cited(n)))]
        print(f"  {label} in-scope check failures: {len(old_in)}; run stops then: {sum(1 for s in old.values() if s == 'run-unsupported')}")
        fixed = [n for n, s in old.items() if s not in CHECK_PASS and n not in oos
                 and status.get(n, "pass") in CHECK_PASS]
        print(f"  {label} check failures now passing their check: {len(fixed)}")
        if label == "round 23":
            for n in sorted(new):
                print(f"  new: {completion[n]} {n}")

    # Program tests that do not run.
    print("\nprogram tests in the completion set:")
    for n in sorted(completion):
        if n.startswith("programs/"):
            print(f"  {completion[n]} {n} :: {rows[n][2][:120]}")

    # Rules whose run tests all fail.
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
                run[r].append(status.get(n, "pass"))
    bad = sorted(r for r in run if not any(s in PASSING for s in run[r]))
    print(f"\n{len(run)} rules with a run test; none passing for {len(bad)}:")
    for r in bad:
        print(" ", r, kinds.get(r), run[r])

    # Rules named in the front end's source, by kind.
    src = "".join(p.read_text() for p in pathlib.Path("frontend/src").rglob("*.rs"))
    for kind in ("reject", "undefined", "unspecified", "impl-defined"):
        ids = [r for r, k in kinds.items() if k == kind]
        named = [r for r in ids if re.search(r"\b" + re.escape(r) + r"\b", src)]
        print(f"{kind}: {len(named)} of {len(ids)} named in frontend/src")

    # File sizes.
    files = sorted(((len(p.read_text().splitlines()), p.as_posix()) for p in pathlib.Path("frontend/src").rglob("*.rs")),
                   reverse=True)
    print(f"\nfrontend/src: {sum(l for l, _ in files)} lines in {len(files)} files; over 2000:",
          [(f.removeprefix('frontend/src/'), l) for l, f in files if l > 2000])


if __name__ == "__main__":
    main()
