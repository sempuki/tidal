"""Lines that one commit of a range adds and a later commit of the range
removes, outside tests/ and generated tables, to find work that a commit
built in a worktree may have dropped. Usage: undone.py BASE HEAD"""
import subprocess, sys, collections
G = ['git', '-C', '/tmp/claude-1000/-home-ryanm-Code-tidal/34cd0ee5-eea0-4b25-b33b-c46b2846603b/scratchpad/w2']
cs = subprocess.check_output(G + ['rev-list', '--reverse', f'{sys.argv[1]}..{sys.argv[2]}']).decode().split()
added = {}
for c in cs:
    d = subprocess.check_output(G + ['show', '--format=', '-U0', c, '--', '.', ':!tests', ':!reports/clean-room/rust-frontend/conformance.tsv']).decode().splitlines()
    f = None
    for l in d:
        if l.startswith('+++ '): f = l[6:]; continue
        if l.startswith('--- '): continue
        if l.startswith('-') and len(l) > 25 and (f, l[1:]) in added and added[(f, l[1:])] != c[:8]:
            print(c[:8], 'removes a line that', added[(f, l[1:])], 'added in', f, '|', l[1:90])
        if l.startswith('+') and len(l) > 25:
            added[(f, l[1:])] = c[:8]
