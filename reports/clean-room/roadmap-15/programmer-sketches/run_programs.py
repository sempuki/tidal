#!/usr/bin/env python3
"""Runs `tidal-next run` and `tidal-next parse` on each program test under
tests/programs/ whose expectation is not reject, and on the last versions
of the roadmap sketches of rounds 8 to 15, and counts the outcomes by
kind (read-only).
Usage: run_programs.py REPO_ROOT TIDAL_NEXT"""
import collections, re, subprocess, sys
from pathlib import Path
root, nx = Path(sys.argv[1]), sys.argv[2]

def first(cmd, paths):
    r = subprocess.run(['timeout', '120', nx, cmd, *map(str, paths)], capture_output=True, text=True)
    return ((r.stdout + r.stderr).strip().split('\n') or [''])[0].split('\t')[0].strip()

def kind(out):
    if out.startswith('result'): return 'result'
    if out.startswith('unsupported'): return 'unsupported'
    if out.startswith('rejected'): return 'rejected'
    if out.startswith('undefined'): return 'undefined'
    return 'other'

def report(label, items):
    runs = collections.Counter(); trees = 0; rejects = []; why = collections.Counter()
    for name, paths in items:
        p = first('parse', paths)
        trees += p.startswith('tree')
        out = first('run', paths)
        k = kind(out); runs[k] += 1
        if k == 'unsupported':
            why[re.sub(r'^unsupported at line \d+: ', '', out)[:70]] += 1
        if k in ('rejected', 'undefined', 'other', 'result'):
            rejects.append((name, out))
    print(f'{label}: {len(items)} files; parse to a tree {trees}; run: {dict(runs)}')
    for name, out in rejects:
        print(f'  {name}: {out[:120]}')
    print('  first unsupported construct, by count:')
    for w, n in why.most_common():
        print(f'    {n:4d}  {w}')

progs = []
for p in sorted((root / 'tests/programs').rglob('*.cpp')):
    if p.name.endswith('.tu.cpp'):
        continue
    t = p.read_text(errors='replace')
    if 'tidal-rules' not in t or re.search(r'tidal-expect:\s*reject', t):
        continue
    m = re.search(r'tidal-units?:\s*(.*)', t)
    units = [p] + [p.parent / u for u in m.group(1).split()] if m else [p]
    progs.append((str(p.relative_to(root / 'tests')), units))
report('program tests (not reject)', progs)
last = []
cr = root / 'reports/clean-room'
for r in range(8, 16):
    d = cr / f'roadmap-{r}' / 'programmer-sketches' / 'new'
    if not d.is_dir():
        continue
    for p in sorted(d.glob('*.cpp')):
        if '_try' in p.name or p.name.startswith(('base_', 'braced_', 'consteval_', 'member_', 'mem_', 'nested_', 'overload_', 'optional_body')):
            continue
        last.append((str(p.relative_to(cr)), [p]))
report('roadmap programs, last versions, rounds 8 to 15', last)
