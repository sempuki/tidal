#!/usr/bin/env python3
"""Read-only scan of tests/ for two forms that the rewrite's parser stops
on in this round's reductions: an attribute after a lambda's captures with
no template head, and a typename specifier used as the type of a
functional cast. Prints each hit with its front-end status.
Usage: scan_grammar.py REPO_ROOT"""
import re, sys
from pathlib import Path
root = Path(sys.argv[1])
status = {}
for line in open(root / 'reports/clean-room/rust-frontend/conformance.tsv'):
    if '\t' in line and not line.startswith('#'):
        s, t = line.rstrip('\n').split('\t', 1)
        status[t] = s
forms = {
    'lambda attribute with no template head': re.compile(r'\[[^\[\]]*\]\s*\[\['),
    'typename specifier as a cast type': re.compile(r'(?<![<,(]\s)(?<!using \w)\btypename\s+[\w:<>, ]+::\w+\s*[{(]'),
}
for p in sorted((root / 'tests').rglob('*.cpp')):
    rel = str(p.relative_to(root / 'tests'))
    text = p.read_text(errors='replace')
    for name, rx in forms.items():
        for m in rx.finditer(text):
            line = text.count('\n', 0, m.start()) + 1
            src = text.split('\n')[line - 1].strip()
            if src.startswith('//'):
                continue
            print(f'{name} | {rel}:{line} | {status.get(rel, "?")} | {src[:100]}')
