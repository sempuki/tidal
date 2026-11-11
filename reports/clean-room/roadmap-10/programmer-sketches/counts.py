#!/usr/bin/env python3
"""Read-only counts for the round 10 programmer report.
Usage: counts.py COMMIT...  Reads each commit's tree with git show and git ls-tree."""
import re, subprocess, sys, collections

def git(*a):
    return subprocess.run(['git', *a], capture_output=True, text=True, check=True).stdout

def files(commit, path):
    return [l for l in git('ls-tree', '-r', '--name-only', commit, path).splitlines()]

def show(commit, p):
    return git('show', f'{commit}:{p}')

for c in sys.argv[1:]:
    rules = withdrawn = 0
    ex = collections.Counter()
    for p in files(c, 'specification'):
        if not p.endswith('.md'):
            continue
        t = show(c, p)
        for m in re.finditer(r'^:::\{rule\} (\S+)\n(.*?)^:::', t, re.M | re.S):
            rules += 1
            if re.search(r'^:kind: withdrawn', m.group(2), re.M):
                withdrawn += 1
        fences = len(re.findall(r'^```cpp', t, re.M))
        marked = 0
        for m in re.finditer(r'^% tidal-example(?::\s*(\S+))?', t, re.M):
            ex[m.group(1) or 'plain'] += 1
            marked += 1
        ex['unmarked'] += fences - marked
    tests = progs = proglines = nsstd = nsstd_prog = 0
    cited = set()
    for p in files(c, 'tests'):
        if not p.endswith('.cpp') or p.endswith('.tu.cpp'):
            continue
        t = show(c, p)
        if 'tidal-rules' not in t:
            continue
        tests += 1
        isprog = p.startswith('tests/programs/')
        if re.search(r'namespace std\b', t):
            nsstd += 1
            nsstd_prog += isprog
        if isprog:
            progs += 1
            for m in re.finditer(r'tidal-rules:(.*)', t):
                cited.update(m.group(1).split())
    for p in files(c, 'tests/programs'):
        if p.endswith('.cpp') or p.endswith('.h'):
            proglines += show(c, p).count('\n')
    print(f'{c}: rules {rules} ({rules - withdrawn} in force, {withdrawn} withdrawn); '
          f'examples {sum(ex.values())} {dict(ex)}; tests {tests}; program tests {progs}; '
          f'lines under tests/programs {proglines}; rules cited by program tests {len(cited)}; '
          f'tests that open namespace std {nsstd} ({nsstd_prog} program tests)')
