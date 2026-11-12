"""Count tests by build-step scope for the round 15 implementer report.

Reads conform-next.txt and conform-old.txt (the `conform -v` output of
tidal-next and tidal-fe at HEAD, in this directory), the takeover list,
the marks file, and the tests' headers. Uses the scope rules of
tools/frontend_check.py (page prefixes from DESIGN-rewrite.md section 3,
the LATER keyword pattern) and adds two step 4 scopes:

  step 4:       every cited rule sits on a step 3 or step 4 page
                (foundations, expressions, statements, declarations,
                classes, overloading, exceptions), and at least one on a
                step 4 page or the test uses a later keyword
  step 4 plain: of these, the tests whose code uses no keyword of the
                templates pages (template, typename, concept, requires)
                and no `std` or #include

Writes scopes.tsv (test, scope, old status, new status, owned) and
prints the counts. Run from the repository root:
    python3 -I reports/clean-room/roadmap-15/implementer-scratch/scopes.py
"""
import re
import sys
from collections import Counter
from pathlib import Path

ROOT = Path.cwd()
HERE = ROOT / "reports/clean-room/roadmap-15/implementer-scratch"
sys.dont_write_bytecode = True
sys.path.insert(0, str(ROOT / "tools"))
import frontend_check as fc  # noqa: E402

TEMPLATE_KW = re.compile(r"\b(template|typename|concept|requires|std)\b|#\s*include")


def statuses(path: Path) -> dict[str, str]:
    out = {}
    for line in path.read_text(errors="replace").splitlines():
        parts = line.split("\t")
        if len(parts) >= 2 and parts[1].endswith(".cpp"):
            out[parts[1]] = parts[0]
    for p in (ROOT / "tests").rglob("*.cpp"):
        n = p.relative_to(ROOT / "tests").as_posix()
        if not n.endswith(".tu.cpp"):
            out.setdefault(n, "pass")
    return out


def main() -> None:
    new = statuses(HERE / "conform-next.txt")
    old = statuses(HERE / "conform-old.txt")
    owned = set(fc.read_takeover()[0])
    marks, _ = fc.rewrite_marks()
    prefixes = fc.page_prefixes()
    s3, s4 = fc.STEP_CHAPTERS["3"], fc.STEP_CHAPTERS["4"]
    rows = []
    for name in sorted(new):
        path = ROOT / "tests" / name
        rules = fc.cited_rules(path)
        chapters = {prefixes.get(r.split("-")[0]) for r in rules}
        sources = [path] + [path.parent / s for s in fc.header_expect(path)[1]]
        code = " ".join(fc.code_text(s) for s in sources if s.exists())
        later = bool(fc.LATER.search(code))
        tmpl = bool(TEMPLATE_KW.search(code))
        if rules and chapters <= s3 and not later:
            scope = "3-plain"
        elif rules and chapters <= s3 and not tmpl:
            scope = "4-plain"      # step 3 rules, but a class, throw, new, ... construct
        elif rules and chapters <= s3:
            scope = "3-templ"      # step 3 rules, template or std construct
        elif rules and chapters <= (s3 | s4) and not tmpl:
            scope = "4-plain"
        elif rules and chapters <= (s3 | s4):
            scope = "4-templ"
        else:
            scope = "5-7"
        rows.append((name, scope, old.get(name, "?"), new[name], name in owned, name in marks))
    with open(HERE / "scopes.tsv", "w") as f:
        f.write("# test\tscope\told\tnew\towned\tmarked\n")
        for r in rows:
            f.write("\t".join(map(str, r)) + "\n")
    ok = lambda s: fc.rank(s) >= 1
    print("scope\ttests\told-check\tnew-check\tboth\tnew-only\towned\tprograms\tprograms-old-run")
    for sc in ("3-plain", "4-plain", "3-templ", "4-templ", "5-7"):
        sel = [r for r in rows if r[1] == sc]
        progs = [r for r in sel if r[0].startswith("programs/")]
        print(sc, len(sel), sum(ok(r[2]) for r in sel), sum(ok(r[3]) for r in sel),
              sum(ok(r[2]) and ok(r[3]) for r in sel), sum(ok(r[3]) and not ok(r[2]) for r in sel),
              sum(r[4] for r in sel), len(progs), sum(fc.rank(r[2]) == 2 for r in progs), sep="\t")
    # The step 4 plain scope split by the chapters of the cited rules.
    s3only = {r[0] for r in rows if r[1] == "4-plain"
              and {prefixes.get(x.split("-")[0]) for x in fc.cited_rules(ROOT / "tests" / r[0])} <= s3}
    for label, pick in (("4-plain, step 3 rules only", lambda r: r[0] in s3only),
                        ("4-plain, a step 4 rule", lambda r: r[0] not in s3only)):
        sel = [r for r in rows if r[1] == "4-plain" and pick(r)]
        print(label, len(sel), sum(ok(r[2]) for r in sel), sum(ok(r[3]) for r in sel), sep="\t")
    # Step 3 plain tests that pass on both and are not owned, by expectation.
    plain_both = [r for r in rows if r[1] == "3-plain" and ok(r[2]) and ok(r[3]) and not r[4]]
    exp = Counter(fc.header_expect(ROOT / "tests" / r[0])[0] for r in plain_both)
    stat = Counter(r[3] for r in plain_both)
    print("\nstep 3 plain, both pass their check, not owned:", len(plain_both), dict(exp), dict(stat))
    # New front end statuses on the step 4 plain scope.
    sel = [r for r in rows if r[1] == "4-plain"]
    print("\nstep 4 plain, new statuses:", dict(Counter(r[3] for r in sel).most_common()))
    print("step 4 plain, by chapter of the test directory:",
          dict(Counter(r[0].split("/")[0] for r in sel).most_common()))
    # Owned tests whose old status is not a passing one (none expected).
    print("\nowned tests the old front end does not pass:", [r[0] for r in rows if r[4] and r[2] not in fc.PASSING])


if __name__ == "__main__":
    main()
