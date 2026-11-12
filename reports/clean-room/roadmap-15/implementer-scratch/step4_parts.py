"""Split the step 4 plain scope (scopes.py) into the parts that a staged
step 4 could take, for the round 15 implementer report.

A test of the step 4 plain scope goes to the first part whose test it
meets, in this order (my judgment):

  4c exceptions:  the code uses throw, try, or catch, or a cited rule
                  sits on an exceptions page
  4b overloading: the code uses virtual or operator, or a cited rule
                  sits on an overloading page or on the classes pages
                  virtual, operators, comparisons, conversion-functions
  4a classes:     every other test (class definitions, members, access,
                  special members, derived classes, unions, new and
                  delete, this, lambdas)

Prints for each part the tests, the old front end's passes of the check,
the new front end's, and the program tests. Run from the repository root.
"""
import re
import sys
from pathlib import Path

ROOT = Path.cwd()
HERE = ROOT / "reports/clean-room/roadmap-15/implementer-scratch"
sys.dont_write_bytecode = True
sys.path.insert(0, str(ROOT / "tools"))
import frontend_check as fc  # noqa: E402

EXC = re.compile(r"\b(throw|try|catch)\b")
OVL = re.compile(r"\b(virtual|operator)\b")
OVL_PAGES = ("classes/virtual", "classes/operators", "classes/comparisons", "classes/conversion-functions")


def page_of_rules() -> dict[str, str]:
    """{rule prefix: page path}, from the page table of DESIGN-rewrite.md section 3."""
    out = {}
    for line in (ROOT / "frontend/DESIGN-rewrite.md").read_text().splitlines():
        m = re.match(r"^\| `([a-z-]+/[^`]+)` \| `[^`]+` \| ([A-Z, ]+) \|", line)
        if m:
            for p in m.group(2).split(","):
                out[p.strip()] = m.group(1)
    return out


def main() -> None:
    pages = page_of_rules()
    rows = [l.split("\t") for l in (HERE / "scopes.tsv").read_text().splitlines() if not l.startswith("#")]
    parts = {"4a classes": [], "4b overloading": [], "4c exceptions": []}
    for r in rows:
        if r[1] != "4-plain":
            continue
        path = ROOT / "tests" / r[0]
        sources = [path] + [path.parent / s for s in fc.header_expect(path)[1]]
        code = " ".join(fc.code_text(s) for s in sources if s.exists())
        rp = {pages.get(x.split("-")[0], "?") for x in fc.cited_rules(path)}
        if EXC.search(code) or any(p.startswith("exceptions/") for p in rp):
            parts["4c exceptions"].append(r)
        elif OVL.search(code) or any(p.startswith("overloading/") or p.startswith(OVL_PAGES) for p in rp):
            parts["4b overloading"].append(r)
        else:
            parts["4a classes"].append(r)
    ok = lambda s: fc.rank(s) >= 1
    print("part\ttests\told-check\tnew-check\tprograms")
    for k, sel in parts.items():
        print(k, len(sel), sum(ok(r[2]) for r in sel), sum(ok(r[3]) for r in sel),
              sum(r[0].startswith("programs/") for r in sel), sep="\t")


if __name__ == "__main__":
    main()
