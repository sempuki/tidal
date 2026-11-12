#!/usr/bin/env python3
"""Compares this round's rerun log with round 14's log of the same files:
the tidal-fe line, the compilers' and EDG's verdicts, and the kind of the
tidal-next run line (read-only).
Usage: compare_rerun.py ROUND14_LOG RERUN_LOG"""
import collections, re, sys
sys.path.insert(0, __import__('os').path.dirname(__file__))
from table import blocks

def norm(v):
    v = (v or '').strip()
    v = re.sub(r'explored: .*', '', v)
    v = re.sub(r' in every explored order.*', '', v)
    return re.sub(r'\s+', ' ', v).strip()

def verdict(b):
    out = []
    for k in ('g++', 'g++ ped', 'clang++', 'clang++ ped', 'EDG'):
        v = b.get(k, '')
        out.append('fail' if v.startswith('FAILED') or v.startswith(' rejects') or 'rejects' in v[:9] else 'ok')
    return tuple(out)

def nkind(v):
    v = (v or '').strip()
    for k in ('result', 'unsupported', 'rejected', 'undefined'):
        if v.startswith(k): return k
    return 'other'

old = {b['file']: b for b in blocks(sys.argv[1])}
same = changed = 0
kinds = collections.Counter(); fe = collections.Counter()
for b in blocks(sys.argv[2]):
    name = b['file'].replace('round14/', '', 1)
    o = old.get(name)
    k = nkind(b.get('tidal-next run')); kinds[k] += 1
    fe[nkind(b.get('tidal-fe'))] += 1
    if o is None:
        print('no round 14 row:', name); continue
    a, c = norm(o.get('tidal-fe')), norm(b.get('tidal-fe'))
    if a == c:
        same += 1
    else:
        changed += 1
        print(f'changed {name}\n  round 14: {a[:150]}\n  now:      {c[:150]}')
    if verdict(o) != verdict(b):
        print(f'compiler verdict changed {name}: {verdict(o)} -> {verdict(b)}')
print(f'tidal-fe: {same} same, {changed} changed; tidal-fe kinds {dict(fe)}; tidal-next run kinds {dict(kinds)}')
