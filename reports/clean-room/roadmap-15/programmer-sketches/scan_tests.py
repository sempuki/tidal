#!/usr/bin/env python3
"""Scans tests/ for the forms of this round's tidal-fe reductions and
prints each hit with its conformance.tsv status (read-only).
Usage: scan_tests.py REPO_ROOT"""
import re, sys
from pathlib import Path
root = Path(sys.argv[1])
status = {}
for line in (root / 'reports/clean-room/rust-frontend/conformance.tsv').read_text().splitlines():
    p = line.split('\t')
    if len(p) >= 2:
        status[p[1]] = p[0]
forms = {
    'a using-declaration of an operator function': re.compile(r'using\s+[\w:]*::operator\s*\S'),
    'a template template parameter pack': re.compile(r'template\s*<[^<>]*>\s*(class|typename)\s*\.\.\.'),
    'a base named with ::template': re.compile(r'(struct|class)\s+\w+[^;{]*:[^;{]*::template\s+\w+\s*<'),
    'a member template of a specialization as a template argument': re.compile(r'<\s*(\w+::)*\w+<[^<>;]*>::\w+\s*>(?!\s*::)'),
    'a constructor template with a parameter of reference to array of deduced bound':
        re.compile(r'template\s*<[^<>]*\b(size_t|decltype\(sizeof 0\)|int|unsigned)\s+(\w+)\s*>\s*(constexpr\s+|explicit\s+)*(\w+)\s*\(\s*(const\s+)?[\w:<>, ]+\(\s*&\s*\w*\s*\)\s*\[\s*\2\s*\]'),
    'tidal-next: a class template whose base clause names the class template with arguments':
        re.compile(r'template\s*<[^{;]*>\s*(struct|class)\s+(\w+)\s*(final\s*)?:[^{;]*\b\2\s*<'),
}
for name, rx in forms.items():
    hits = []
    for p in sorted((root / 'tests').rglob('*.cpp')):
        t = p.read_text(errors='replace')
        if rx.search(t):
            rel = str(p.relative_to(root / 'tests'))
            hits.append(f'{rel} ({status.get(rel, "no row")})')
    print(f'{name}: {len(hits)} tests')
    for h in hits[:40]:
        print('  ' + h)
