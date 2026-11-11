#!/usr/bin/env python3
"""Prints one line per file of a results log: the GCC and Clang -Wall
results, EDG, and tidal-fe, and a count of tidal-fe results by kind.
Usage: summarize.py results-new.txt"""
import collections, sys
kinds = collections.Counter()
f = None
row = {}
for l in open(sys.argv[1]):
    l = l.rstrip('\n')
    if l.startswith('=== '):
        f = l[4:]; row = {}
    elif l.startswith('g++: '): row['gcc'] = l[5:45]
    elif l.startswith('clang++: '): row['clang'] = l[9:49]
    elif l.startswith('EDG:'): row['edg'] = l[4:30].strip()
    elif l.startswith('tidal-fe: ') and f:
        fe = l[10:]
        kind = fe.split()[0].rstrip(':') if fe else 'none'
        group = f.split('/')[-2] if '/' in f else 'top'
        kinds[(group, kind)] += 1
        print(f'{f} | gcc {row.get("gcc")} | clang {row.get("clang")} | edg {row.get("edg")} | fe {fe[:120]}')
for (g, k), n in sorted(kinds.items()):
    print(f'count {g} {k}: {n}')
