"""Sort the tests by the build step whose pages hold every rule they cite.

A test belongs to step 3 when every rule ID on its `tidal-rules:` line has
a prefix of a step 3 page (foundations, expressions, statements,
declarations; DESIGN-rewrite.md section 3), to step 4 when every prefix is
of a step 3 or step 4 page (classes, overloading, exceptions), and to the
templates steps otherwise. A test with no rules line is "none". Joins each
group with the round 27 statuses and with parse-causes.tsv.

Run from the repository root after parse_causes.py:
    python3 -I reports/clean-room/roadmap-14/implementer-scratch/step_scope.py
"""
from collections import Counter
from pathlib import Path

ROOT = Path.cwd()
HERE = ROOT / "reports/clean-room/roadmap-14/implementer-scratch"
STEP3 = set("TR KW NAM INC ODR TYP VC CONV OBJ STG SEQ CE LSUP PRIM PFX UN CAST MPTR MUL ADD SHF CMP BIT "
            "LOG COND ASN COMMA DTY UNEV LAM NEW STMT IF SW LOOP RFOR JMP CV DCL TYN ATTR ALN AUTO CEV CINIT "
            "CX ENUM EXS FN DEFARG DEL RET INIT LIST REF SINIT INL LSP NS SA LNK STO SB VOL".split())
STEP4 = set("CLS ACC DER VIRT CTOR DTOR CPY CVF ILC OPR DCMP BF UNI ICS OVL THR TRY UNW".split())
TEMPL = set("TPL TARG DEP PACK INST TSP DED ORD CPT CNS CTAD".split())
PASSING = {"pass", "EDG-ONLY", "EDG-ALLOWED", "pass-partial", "NO-DIAGNOSTIC-ALLOWED"}

status = {}
for line in (ROOT / "reports/clean-room/rust-frontend/conformance.tsv").read_text().splitlines():
    if line and not line.startswith("#"):
        s, n = line.split("\t")
        status[n] = s
parse = {}
for line in (HERE / "parse-causes.tsv").read_text().splitlines():
    t, kind, group, *_ = line.split("\t")
    parse[t] = (kind, group)

unknown = Counter()
groups: dict[str, list[str]] = {"3": [], "4": [], "templates": [], "none": []}
for test in sorted(status):
    rules = set()
    for ln in (ROOT / "tests" / test).read_text(errors="replace").splitlines():
        if not ln.startswith("//"):
            break
        if ln.startswith("// tidal-rules:"):
            rules |= set(ln.split(":", 1)[1].split())
    prefixes = {r.split("-")[0] for r in rules}
    for p in prefixes - STEP3 - STEP4 - TEMPL:
        unknown[p] += 1
    if not prefixes:
        groups["none"].append(test)
    elif prefixes <= STEP3:
        groups["3"].append(test)
    elif prefixes <= STEP3 | STEP4:
        groups["4"].append(test)
    else:
        groups["templates"].append(test)

print("| Tests whose rules all sit on pages of | Tests | Old front end passes its check | Program tests | "
      "Parse disagreements (tree side, reject side) |")
print("|---|---|---|---|---|")
for g, ts in groups.items():
    ok = sum(status[t] in PASSING or status[t].startswith("run-") for t in ts)
    progs = sum(t.startswith("programs/") for t in ts)
    tree = sum(t in parse and parse[t][0] != "reject" for t in ts)
    rej = sum(t in parse and parse[t][0] == "reject" for t in ts)
    print(f"| {g} | {len(ts)} | {ok} | {progs} | {tree}, {rej} |")
if unknown:
    print(f"prefixes in no step: {dict(unknown)}")

# The tests of each step's scope that the old front end does not pass its
# check on, with their status and whether the ledger has them out of scope.
for g in ("3", "4"):
    bad = [t for t in groups[g] if not (status[t] in PASSING or status[t].startswith("run-"))]
    print(f"\nstep {g} scope, old front end fails its check: {len(bad)}")
    for t in bad:
        print(f"  {status[t]}\t{t}")
runs = [t for t in status if status[t] == "run-unsupported"]
print(f"\nrun stops by step scope: " + ", ".join(f"{g} {sum(t in set(groups[g]) for t in runs)}" for g in groups))

# The step 3 tests whose text, comments and literals removed, has none of
# the keywords of a later step's pages. This is a floor for the step 3
# shadow line: these tests need no class, template, or exception code.
import re as _re
LATER = _re.compile(r"\b(class|struct|union|template|typename|concept|requires|throw|try|catch|operator|"
                    r"virtual|friend|this|new|delete|std)\b|#\s*include")


def code(path: Path) -> str:
    t = path.read_text(errors="replace")
    t = _re.sub(r"//[^\n]*|/\*.*?\*/", " ", t, flags=_re.S)
    return _re.sub(r'"(\\.|[^"\\\n])*"|\'(\\.|[^\'\\\n])*\'', " ", t)


plain = []
for t in groups["3"]:
    p = ROOT / "tests" / t
    srcs = [p]
    for ln in p.read_text(errors="replace").splitlines():
        if ln.startswith("// tidal-sources:"):
            srcs += [p.parent / s for s in ln.split(":", 1)[1].split()]
    if not any(LATER.search(code(s)) for s in srcs):
        plain.append(t)
ok = sum(status[t] in PASSING or status[t].startswith("run-") for t in plain)
print(f"\nstep 3 tests with no keyword of a later step: {len(plain)}; the old front end passes its check on {ok}; "
      f"parse disagreements {sum(t in parse and parse[t][0] != "reject" for t in plain)} on the tree side and {sum(t in parse and parse[t][0] == "reject" for t in plain)} reject tests; program tests {sum(t.startswith('programs/') for t in plain)}")
