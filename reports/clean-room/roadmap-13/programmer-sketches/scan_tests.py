#!/usr/bin/env python3
"""Scans tests/ (read-only) for the forms of this round's reductions and
prints each hit with the front end's status from conformance.tsv.
Usage: scan_tests.py REPO_ROOT. The patterns are rough; each hit needs a read."""
import re, sys
from pathlib import Path

root = Path(sys.argv[1])
status = {}
for line in open(root / 'reports/clean-room/rust-frontend/conformance.tsv'):
    if not line.startswith('#') and '\t' in line:
        s, t = line.rstrip('\n').split('\t', 1)
        status[t] = s

forms = {
    'fold in a requires clause': re.compile(r'requires\s*\((?:[^()]|\([^()]*\))*(?:&&|\|\|)\s*\.\.\.\s*\)|requires\s*\(\s*\.\.\.\s*(?:&&|\|\|)'),
    'partial specialization with a braced constant argument': re.compile(r'^\s*template\s*<[^>]*>\s*(?:struct|class)\s+\w+\s*<[^;{]*\w\{[^}]*\}', re.M),
    'conversion function with a trailing requires clause': re.compile(r'operator\s+[\w:<>]+\s*\(\s*\)\s*(?:const\s*)?(?:&\s*)?requires'),
    'copy constructor with a trailing requires clause': re.compile(r'\b(\w+)\s*\(\s*const\s+\1\s*(?:<[^>]*>)?\s*&\s*\w*\s*\)\s*requires'),
    'conversion function template': re.compile(r'template\s*<[^>]*>\s*(?:constexpr\s+)?(?:explicit\s+)?operator\s+[A-Z]\w*\s*\(\s*\)'),
    'decltype as a qualifier': re.compile(r'decltype\s*\((?:[^()]|\([^()]*\))*\)\s*::'),
    'deleted function template at namespace scope': re.compile(r'^template\s*<[^>]*>\s*[\w:&*<> ]+\s+\w+\s*\([^)]*\)\s*=\s*delete\s*;', re.M),
    'friend class template': re.compile(r'template\s*<[^>]*>\s*friend\s+(?:class|struct)\s+(\w+)'),
}

def declared_before(text, name, pos):
    return re.search(r'(?:class|struct)\s+' + re.escape(name) + r'\b', text[:pos]) is not None

for p in sorted((root / 'tests').rglob('*.cpp')):
    rel = str(p.relative_to(root / 'tests'))
    text = p.read_text(errors='replace')
    for name, rx in forms.items():
        for m in rx.finditer(text):
            if name == 'friend class template':
                # Only a friend that names a class template declared later.
                if declared_before(text, m.group(1), m.start()):
                    continue
            line = text.count('\n', 0, m.start()) + 1
            print(f'{name} | {rel}:{line} | {status.get(rel, "no row")}')
            break
