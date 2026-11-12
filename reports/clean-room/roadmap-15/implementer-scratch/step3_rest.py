"""Sort the step 3 plain tests that the old front end passes and the new
one does not, for the round 15 implementer report.

Reads scopes.tsv (written by scopes.py) and each test's source. A test
goes to the first group whose pattern its code matches, in the order of
GROUPS; the patterns are my judgment, and they follow the cause table of
report-rw3 section 2. Missed rejects and EDG-AGREES tests are also
counted by the page directory of the test and by the prefix of their
first cited rule. Writes step3-rest.tsv and prints the counts. Run from
the repository root.
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

GROUPS = [
    ("lambda", re.compile(r"\[[^\]\[]*\]\s*(\([^)]*\))?\s*(mutable|constexpr|consteval|noexcept|->|\{)")),
    ("structured binding", re.compile(r"\bauto\s*&{0,2}\s*\[")),
    ("if consteval", re.compile(r"\bif\s*!?\s*consteval\b")),
    ("noexcept", re.compile(r"\bnoexcept\s*\(")),
    ("assume", re.compile(r"\[\[\s*assume\b")),
    ("long double", re.compile(r"\blong\s+double\b|\d[lL]\b")),
]


def main() -> None:
    rows = [l.split("\t") for l in (HERE / "scopes.tsv").read_text().splitlines() if not l.startswith("#")]
    sel = [r for r in rows if r[1] == "3-plain" and fc.rank(r[2]) >= 1 and fc.rank(r[3]) == 0]
    groups: dict[str, list] = defaultdict(list)
    out = []
    for name, _, old, new, *_ in sel:
        path = ROOT / "tests" / name
        sources = [path] + [path.parent / s for s in fc.header_expect(path)[1]]
        code = " ".join(fc.code_text(s) for s in sources if s.exists())
        if new in ("MISSED-REJECT", "EDG-AGREES"):
            g = "missing check" if new == "MISSED-REJECT" else "missing check (EDG-AGREES)"
        else:
            g = next((label for label, pat in GROUPS if pat.search(code)), f"other ({new})")
        groups[g].append(name)
        out.append((name, new, g))
    with open(HERE / "step3-rest.tsv", "w") as f:
        f.write("# test\tnew status\tgroup\n")
        for r in out:
            f.write("\t".join(r) + "\n")
    print(f"{len(sel)} tests")
    for g, names in sorted(groups.items(), key=lambda kv: -len(kv[1])):
        print(f"{len(names)}\t{g}")
    checks = groups["missing check"] + groups["missing check (EDG-AGREES)"]
    by_dir = Counter("/".join(n.split("/")[:2]) for n in checks)
    print("\nmissing checks by page directory:")
    for d, c in by_dir.most_common():
        print(f"{c}\t{d}")
    by_rule = Counter()
    for n in checks:
        rules = sorted(fc.cited_rules(ROOT / "tests" / n))
        by_rule[rules[0].split("-")[0] if rules else "?"] += 1
    print("\nmissing checks by prefix of the first cited rule:")
    print(", ".join(f"{p} {c}" for p, c in by_rule.most_common()))


if __name__ == "__main__":
    main()
