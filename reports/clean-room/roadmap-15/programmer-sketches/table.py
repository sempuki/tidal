#!/usr/bin/env python3
"""Prints one line per file of a results log: the host compilers'
verdict, EDG's, tidal-fe's, and tidal-next run's.
Usage: table.py LOG"""
import sys, re

def blocks(path):
    cur = None
    for line in open(path):
        line = line.rstrip('\n')
        if line.startswith('=== '):
            if cur: yield cur
            cur = {'file': line[4:]}
        elif cur is not None and ': ' in line:
            k, v = line.split(': ', 1)
            cur[k] = v
    if cur: yield cur

def short(v, n=90):
    return (v or '').strip()[:n]

def cc(b, name):
    v = b.get(name, '')
    if v.startswith('FAILED'): return 'fail'
    m = re.search(r'exit (\d+)', v)
    return 'exit ' + m.group(1) if m else '?'

if __name__ == '__main__':
    for b in blocks(sys.argv[1]):
        print('\t'.join([b['file'], 'g:' + cc(b, 'g++'), 'c:' + cc(b, 'clang++'),
                         'gs:' + cc(b, 'g++ san'), 'cs:' + cc(b, 'clang++ san'),
                         'EDG:' + short(b.get('EDG'), 20),
                         'FE:' + short(b.get('tidal-fe')),
                         'NX:' + short(b.get('tidal-next run')),
                         'P:' + short(b.get('tidal-next parse'), 60)]))
