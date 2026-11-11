"""For each rule that rests only on tests with an override for a newest
compiler: its class, its tests, the override keys, and the questions the
test notes name. Read-only. Run from the repository root."""
import re, sys
sys.path.insert(0, "tools")
import check_specification as cs
from run_tests import parse_test
rules, _ = cs.parse_spec()
thin = cs.thin_evidence(rules)
newest = cs.newest_overrides()
fam = {}
for rid, why in sorted(thin.items()):
    if "override" not in why:
        continue
    f, cls, tests = rules[rid]
    keys, qs = set(), set()
    for t in tests:
        p = parse_test(cs.TESTS / t)
        keys |= set(p.overrides) & newest
        qs |= set(re.findall(r"Q-\d{4}", " ".join(p.notes)))
        txt = (cs.TESTS / t).read_text()
        qs |= set(re.findall(r"Q-\d{4}", "\n".join(l for l in txt.splitlines() if l.startswith("//"))))
    k = ",".join(sorted(keys))
    fam[k] = fam.get(k, 0) + 1
    print(f"{rid}\t{cls}\t{len(tests)} tests\t{k}\t{' '.join(sorted(qs))}")
print(fam)
