"""Print the productions of the Syntax segments that the named nonterminals
have, with their page. Read-only. Run from the repository root.
With no arguments, print the count of nonterminals and pages."""
import re, sys, pathlib
prods = {}
for f in sorted(pathlib.Path("specification").rglob("*.md")):
    t = f.read_text()
    for block in re.findall(r"\.\. productionlist:: tidal\n(.*?)```", t, re.S):
        name = None
        for line in block.splitlines():
            m = re.match(r"\s+(\w+):\s?(.*)$", line)
            if m and not line.strip().startswith(":"):
                name = m.group(1)
                prods.setdefault(name, (str(f), []))[1].append(m.group(2))
            elif name and line.strip().startswith(":"):
                prods[name][1].append(line.strip()[1:].strip())
if len(sys.argv) == 1:
    print(len(prods), "nonterminals on", len({p for p, _ in prods.values()}), "pages")
for n in sys.argv[1:]:
    if n in prods:
        f, alts = prods[n]
        print(f"{n} ({f[len('specification/'):]}): " + "\n    ".join(alts))
    else:
        print(f"{n}: none")
