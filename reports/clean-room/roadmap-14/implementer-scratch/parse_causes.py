"""Sort the parse count's disagreements by the G row of report-rw1.md.

Reads check-next-output.txt (the output of frontend_check.py --next), takes
the line of each stop from the test file, and puts each disagreement in the
first group whose pattern fits that line. The patterns are my judgment; a
disagreement that no pattern fits is "other". Prints a table of groups by
the test's base expectation, and writes parse-causes.tsv with one row per
disagreement.

Run from the repository root:
    python3 -I reports/clean-room/roadmap-14/implementer-scratch/parse_causes.py
"""
import re
import sys
from collections import Counter
from pathlib import Path

ROOT = Path.cwd()
HERE = ROOT / "reports/clean-room/roadmap-14/implementer-scratch"
OUT = (HERE / "check-next-output.txt").read_text().splitlines()

TID = r"[A-Za-z_][\w:]*\s*<[^;]*?>"
GROUPS = [
    ("G169 using enum", r"\busing\s+enum\b"),
    ("G168 using-directive or alias, unqualified", r"\busing\s+namespace\s+\w+\s*;|\bnamespace\s+\w+\s*=\s*[\w:]+\s*;"),
    ("G175 throw()", r"\bthrow\s*\(\s*\)"),
    ("G174 structured binding in init-statement", r"\b(if|switch)\s*\(\s*(const\s+)?auto\s*&{0,2}\s*\["),
    ("G172 opaque enum in a class", r"^\s*enum\s+(class\s+|struct\s+)?\w+\s*:\s*[\w ]+;"),
    ("G173 member template with no decl-specifiers", r"template\s*<.*>\s*(explicit\s+)?operator\s+\w"),
    ("G177 pointer to member in conversion_type", r"operator\s+[\w ]+\s+\w+::\*"),
    ("G171 operator or destructor template-id", r"operator\s*(\(\)|\"\"\s*\w+|\w+)\s*<|~\s*\w+\s*<|~\s*decltype"),
    ("G178 braced template argument", r"<\s*\{"),
    ("G176 static_assert with two literals", r"static_assert\s*\(.*\"\s*\"[^)]*\)"),
    ("G167 base or mem-initializer template-id or decltype",
     r"(class|struct)\s+\w+[^;{(]*:\s*(public\s+|private\s+|protected\s+|virtual\s+)*(" + TID + r"|decltype)"
     r"|\)\s*:\s*(" + TID + r"\s*[({])|,\s*" + TID + r"\s*[({]|:\s*decltype\s*\("),
    ("G166 template-id cast with braced list", TID + r"\s*\{"),
    ("G179 template and a name with no list", r"\btemplate\s+\w+\s*[>,;=)]|typename\s+[\w:]+::template\s+\w+\s*[^<\w]"),
    ("G170 attribute position", r"\[\["),
]

rows = []
for line in OUT:
    m = re.match(r"next: parse: (\S+): wants a tree, the unit gives stop (\d+): (.*)", line)
    if m:
        rows.append((m.group(1), "tree", int(m.group(2)), m.group(3)))
        continue
    m = re.match(r"next: parse: (\S+): wants a stop \(TR-12\), gives (.*)", line)
    if m:
        rows.append((m.group(1), "stop", 0, m.group(2)))


def expect(test: str) -> str:
    for ln in (ROOT / "tests" / test).read_text(errors="replace").splitlines():
        if not ln.startswith("//"):
            break
        if ln.startswith("// tidal-expect:"):
            return ln.split(":", 1)[1].split()[0]
    return "?"


table: Counter = Counter()
lines_out = []
for test, want, lineno, msg in rows:
    kind = expect(test)
    side = "reject" if kind == "reject" else "tree"
    if want == "stop":
        group = "tree where the old first error names TR-12"
    else:
        text = (ROOT / "tests" / test).read_text(errors="replace").splitlines()
        src = text[lineno - 1] if 0 < lineno <= len(text) else ""
        # A stop can fall on the line after the form; read two lines.
        near = src + " " + (text[lineno] if lineno < len(text) else "")
        group = "other"
        for name, pat in GROUPS:
            if re.search(pat, src):
                group = name
                break
        else:
            for name, pat in GROUPS:
                if re.search(pat, near):
                    group = name
                    break
    table[(group, side)] += 1
    lines_out.append(f"{test}\t{kind}\t{group}\t{lineno}\t{msg}")

(HERE / "parse-causes.tsv").write_text("\n".join(lines_out) + "\n")
groups = sorted({g for g, _ in table}, key=lambda g: (g == "other", g))
print("| Cause | Need a tree (accept, run, run-fail, link-fail) | Reject tests | All |")
print("|---|---|---|---|")
for g in groups:
    a, r = table[(g, "tree")], table[(g, "reject")]
    print(f"| {g} | {a} | {r} | {a + r} |")
print(f"| total | {sum(v for (g, s), v in table.items() if s == 'tree')} | "
      f"{sum(v for (g, s), v in table.items() if s == 'reject')} | {len(rows)} |")
sys.exit(0)
