#!/usr/bin/env python3
"""Sorts each file of a results log by the compilers' verdict and by the
result of each front end (read-only).
The compilers' verdict: 'invalid' when g++ or clang++ rejects the file
with -pedantic-errors; 'bug' when the file is under bugs/, or both build
and a sanitizer build stops it; 'valid' when both build and exit 0
without sanitizers, and no sanitizer stops it; 'other' for the rest.
Usage: summarize.py LOG"""
import collections, re, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from table import blocks

def ex(v):
    m = re.search(r'exit (\d+)', v or '')
    return int(m.group(1)) if m else None

def compilers(b):
    if any((b.get(k, '')).startswith('FAILED') for k in ('g++ ped', 'clang++ ped')):
        return 'invalid'
    if '/bugs/' in b['file']:
        return 'bug'
    plain = [ex(b.get(k)) for k in ('g++ ped', 'clang++ ped')]
    san = [ex(b.get(k)) for k in ('g++ san', 'clang++ san')]
    if any(s not in (None, 0) for s in san):
        return 'bug'
    if all(p == 0 for p in plain):
        return 'valid'
    return 'other (exit %s)' % plain

def kind(v):
    v = (v or '').strip()
    for k in ('result', 'unsupported', 'rejected', 'undefined'):
        if v.startswith(k):
            if k == 'result':
                m = re.match(r'result: (-?\d+)', v)
                return 'result 0' if m and m.group(1) == '0' else 'result other'
            return k
    return 'other'

tab = {fe: collections.Counter() for fe in ('tidal-fe', 'tidal-next run')}
odd = []
for b in blocks(sys.argv[1]):
    c = compilers(b)
    for fe in tab:
        k = kind(b.get(fe))
        tab[fe][(c, k)] += 1
        if (c == 'valid' and k in ('rejected', 'undefined', 'result other')) or (c == 'bug' and k.startswith('result')) or (c == 'invalid' and k.startswith('result')):
            odd.append(f'{fe}: {b["file"]}: compilers {c}, {k}: {(b.get(fe) or "")[:110]}')
for fe, t in tab.items():
    print(fe)
    for (c, k), n in sorted(t.items()):
        print(f'  {c:10s} {k:14s} {n}')
print('Files where a front end disagrees with the compilers:')
for o in odd:
    print('  ' + o)
