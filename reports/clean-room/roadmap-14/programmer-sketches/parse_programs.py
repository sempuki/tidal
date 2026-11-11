#!/usr/bin/env python3
"""Runs `tidal-next parse` on each program test under tests/programs/ whose
expectation is not reject, and on the last versions of the roadmap sketches
of rounds 8 to 14, and counts trees and stops (read-only).
Usage: parse_programs.py REPO_ROOT TIDAL_NEXT"""
import re, subprocess, sys
from pathlib import Path
root, nx = Path(sys.argv[1]), sys.argv[2]
def parse(paths):
    r = subprocess.run([nx, 'parse', *map(str, paths)], capture_output=True, text=True)
    return (r.stdout + r.stderr).split('\t')[0].strip()
def report(label, items):
    trees, stops = 0, []
    for name, paths in items:
        out = parse(paths)
        if out.startswith('tree'):
            trees += 1
        else:
            stops.append((name, out))
    print(f'{label}: {trees}/{len(items)} parse to a tree')
    for name, out in stops:
        print(f'  stop {name}: {out[:110]}')
progs = []
for p in sorted((root / 'tests/programs').rglob('*.cpp')):
    if p.name.endswith('.tu.cpp'):
        continue
    t = p.read_text(errors='replace')
    if 'tidal-rules' not in t or re.search(r'tidal-expect:\s*reject', t):
        continue
    units = [p] + [p.parent / u for u in re.findall(r'tidal-units?:\s*(.*)', t)[0].split()] if re.search(r'tidal-units?:', t) else [p]
    progs.append((str(p.relative_to(root / 'tests')), units))
report('program tests (not reject)', progs)
last = []
cr = root / 'reports/clean-room'
for r in range(8, 15):
    d = cr / f'roadmap-{r}' / 'programmer-sketches' / 'new'
    if not d.is_dir():
        continue
    for p in sorted(d.glob('*.cpp')):
        if '_try' in p.name or p.name.startswith(('base_', 'braced_', 'consteval_', 'member_', 'mem_', 'nested_', 'overload_', 'optional_body')):
            continue
        last.append((str(p.relative_to(cr)), [p]))
report('roadmap programs, last versions, rounds 8 to 14', last)
