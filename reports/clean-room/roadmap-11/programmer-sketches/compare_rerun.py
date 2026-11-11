#!/usr/bin/env python3
"""Compares the tidal-fe line of each rerun file (results-rerun.txt) with
round 10's logs (roadmap-10/programmer-sketches/results-new.txt and
results-probes.txt). Prints each change and the count of unchanged files."""
import re
from pathlib import Path
here = Path(__file__).resolve().parent
r10 = here.parent.parent / 'roadmap-10' / 'programmer-sketches'

def fe_lines(path, prefix=''):
    d, f = {}, None
    for l in open(path):
        if l.startswith('=== '):
            f = prefix + l.split()[1]
        elif l.startswith('tidal-fe:') and f:
            d[f] = l[10:].strip()
    return d

def norm(v):
    return re.sub(r'explored.*', '', v)[:110]

old = fe_lines(r10 / 'results-new.txt', 'round10/')
old.update(fe_lines(r10 / 'results-probes.txt', 'round10/'))
new = fe_lines(here / 'results-rerun.txt')
same = changed = 0
for k, v in new.items():
    if norm(old.get(k, '')) == norm(v):
        same += 1
    else:
        changed += 1
        print(f'CHANGED {k}\n  was: {old.get(k, "(none)")[:150]}\n  now: {v[:150]}')
print(f'{len(new)} files: {same} unchanged, {changed} changed')
