#!/usr/bin/env python3
"""Scans tests/ (read-only) for the forms of round 14's reductions and
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
    'generic lambda with a reference capture': re.compile(r'\[\s*&[^\]]*\]\s*(?:<[^>]*>\s*)?\(\s*(?:const\s+)?auto'),
    'static constexpr data member initialized by a call of a static member': re.compile(r'static\s+constexpr\s+[\w:<>]+\s+\w+\s*=\s*(\w+)\s*\(\s*\)\s*;'),
    'requires expression with a braced list of a type': re.compile(r'requires\s*(?:\([^)]*\))?\s*\{\s*(?:\w+::)*[A-Z]\w*\s*\{'),
    'constructor template from a forwarding reference': re.compile(r'template\s*<\s*(?:class|typename)\s+(\w+)\s*>\s*(?:constexpr\s+)?(?:explicit\s+)?([A-Z]\w*)\s*\(\s*\1\s*&&'),
    'class template as a constant parameter type': re.compile(r'template\s*<\s*([A-Z]\w*)\s+\w+\s*>'),
}

def declared_before(text, name, pos):
    return re.search(r'(?:class|struct)\s+' + re.escape(name) + r'\b', text[:pos]) is not None

for p in sorted((root / 'tests').rglob('*.cpp')):
    rel = str(p.relative_to(root / 'tests'))
    text = p.read_text(errors='replace')
    for name, rx in forms.items():
        for m in rx.finditer(text):
            if name == 'static constexpr data member initialized by a call of a static member':
                # Only a call of a static member function of the same class.
                if not re.search(r'static\s+constexpr\s+[\w:<>]+\s+' + re.escape(m.group(1)) + r'\s*\(', text):
                    continue
            if name == 'class template as a constant parameter type':
                # Only a parameter type that is a class template of the file.
                if not re.search(r'template\s*<[^>]*>\s*struct\s+' + re.escape(m.group(1)) + r'\b', text):
                    continue
            line = text.count('\n', 0, m.start()) + 1
            print(f'{name} | {rel}:{line} | {status.get(rel, "no row")}')
            break
