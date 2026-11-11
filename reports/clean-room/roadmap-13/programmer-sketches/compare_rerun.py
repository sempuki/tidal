#!/usr/bin/env python3
"""Compares the tidal-fe line of each rerun file (results-rerun.txt) with
round 12's log (roadmap-12/programmer-sketches/results-new.txt). Also
compares the GCC, Clang, and EDG lines. Prints each change and the counts."""
import re
from pathlib import Path
here = Path(__file__).resolve().parent
r12 = here.parent.parent / 'roadmap-12' / 'programmer-sketches'

def lines(path, prefix=''):
    d, f = {}, None
    for l in open(path):
        l = l.rstrip('\n')
        if l.startswith('=== '):
            f = prefix + l.split()[1]
            d[f] = {}
        elif f and ':' in l:
            k, v = l.split(':', 1)
            d[f][k] = v.strip()
    return d

def norm(v):
    v = re.sub(r'explored.*', '', v or '')
    v = re.sub(r' in every explored order.*', '', v)
    v = re.sub(r'^unspecified: ', '', v)
    return v[:100]

old = lines(r12 / 'results-new.txt', 'round12/')
new = lines(here / 'results-rerun.txt')
same = changed = cc = 0
for k, v in new.items():
    o = old.get(k, {})
    if norm(o.get('tidal-fe')) == norm(v.get('tidal-fe')):
        same += 1
    else:
        changed += 1
        print(f'CHANGED {k}\n  was: {o.get("tidal-fe", "(none)")[:150]}\n  now: {v.get("tidal-fe")[:150]}')
    for t in ('g++', 'clang++', 'EDG'):
        a = re.sub(r'exit \d+.*', '', o.get(t, ''))[:40]
        b = re.sub(r'exit \d+.*', '', v.get(t, ''))[:40]
        if a != b:
            cc += 1
            print(f'COMPILER {k} {t}\n  was: {o.get(t)}\n  now: {v.get(t)}')
print(f'{len(new)} files: {same} unchanged, {changed} changed; {cc} compiler lines differ apart from exit codes')
