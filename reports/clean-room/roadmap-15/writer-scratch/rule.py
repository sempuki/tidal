"""Print the text of the named rules and principles. Read-only."""
import re, sys, pathlib
want = set(sys.argv[1:])
for f in sorted(pathlib.Path("specification").rglob("*.md")):
    t = f.read_text()
    for m in re.finditer(r'^:::\{(rule|principle)\} (\S+)\n(.*?)^:::\s*$', t, re.S | re.M):
        if m.group(2) in want:
            print(f"== {m.group(2)} ({f})")
            print("\n".join(l for l in m.group(3).splitlines() if not l.startswith(":tests:")))
