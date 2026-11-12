#!/usr/bin/env python3
"""Counts the rows of both ledgers by state (read-only)."""
import collections, os
base = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..')
for name, first in (('ledger.md', None), ('rust-frontend/ledger.md', 'G')):
    c = collections.Counter(); opened = []
    for l in open(os.path.join(base, name)):
        p = [x.strip() for x in l.split('|')]
        if len(p) > 4 and p[1] and p[1] != 'Finding' and not p[1].startswith('-') and (first is None or p[1].startswith(first)):
            c[p[3]] += 1
            if p[3] == 'open': opened.append(p[1])
    print(name, sum(c.values()), dict(c)); print('  open:', ' '.join(opened))
