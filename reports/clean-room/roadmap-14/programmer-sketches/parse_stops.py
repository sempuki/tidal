#!/usr/bin/env python3
"""Lists each file of a results log where tidal-next parse stops, with
whether g++ built it, the stop line, and the source text of that line.
Usage: parse_stops.py LOG BASE  (BASE is the directory the log's paths are under)"""
import re, sys
from pathlib import Path
log, base = sys.argv[1], Path(sys.argv[2])
f = None; row = {}
rows = []
for l in open(log):
    l = l.rstrip('\n')
    if l.startswith('=== '):
        f = l[4:]; row = {'f': f}; rows.append(row)
    elif l.startswith('g++: '): row['gcc'] = 'built' if 'built' in l else 'fails'
    elif l.startswith('tidal-next parse: '): row['nx'] = l[18:]
stops = 0
for r in rows:
    nx = r.get('nx', '')
    if not nx.startswith('stop'):
        continue
    stops += 1
    m = re.match(r'stop (\d+): (.*)', nx)
    n = int(m.group(1))
    p = base / r['f']
    src = p.read_text().split('\n')[n - 1].strip() if p.exists() else '?'
    print(f"{r['f']} | g++ {r.get('gcc')} | line {n}: {src[:90]} | {m.group(2)[:90]}")
trees = sum(1 for r in rows if r.get('nx', '').startswith('tree'))
print(f'{len(rows)} files: {trees} tree, {stops} stop')
