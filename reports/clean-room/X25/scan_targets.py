"""Lexical scan for X25: constructions whose target names an element.

Lists each std::construct_at call and each placement new whose first
argument subscripts something or adds to something, in files that also
declare a union. The hits are then checked by hand for an array member
of a union.
"""
import re, sys, pathlib
pat = re.compile(r'(construct_at\s*\(|\bnew\s*\()([^;]*)')
for root in sys.argv[1:]:
    for p in sorted(pathlib.Path(root).rglob('*')):
        if p.suffix not in ('.cpp', '.h', '.hpp'): continue
        t = p.read_text(errors='replace')
        t = re.sub(r'//[^\n]*', '', t)
        if not re.search(r'\bunion\b', t): continue
        for n, line in enumerate(t.split('\n'), 1):
            for m in pat.finditer(line):
                arg = m.group(2)
                if '[' in arg or '+' in arg:
                    print(f'{p}:{n}: {line.strip()}')
