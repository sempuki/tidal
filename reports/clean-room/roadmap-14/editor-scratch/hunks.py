"""Classify the changed lines of each specification file between two commits:
inside a rule or principle block, in the Evidence segment, or elsewhere
(examples, tables outside blocks, glossary). Usage: hunks.py OLD NEW"""
import subprocess, sys, re
G = ['git', '-C', '/tmp/claude-1000/-home-ryanm-Code-tidal/34cd0ee5-eea0-4b25-b33b-c46b2846603b/scratchpad/rv9c']
old, new = sys.argv[1], sys.argv[2]
files = subprocess.check_output(G + ['diff', '--name-only', old, new, '--', 'specification']).decode().split()
for f in files:
    try:
        text = subprocess.check_output(G + ['show', f'{new}:{f}']).decode().splitlines()
    except subprocess.CalledProcessError:
        continue
    where = []
    blk, sect, ev = None, '', False
    for l in text:
        m = re.match(r'^:::\{(rule|principle)\} (\S+)', l)
        if m: blk = m.group(2)
        if l.startswith('## '): sect = l[3:].strip(); ev = (sect == 'Evidence')
        if l.startswith('### '): sub = l[4:].strip()
        where.append(blk if blk else ('Evidence' if ev else sect or 'top'))
        if l.strip() == ':::' and blk: blk = None
    d = subprocess.check_output(G + ['diff', '-U0', old, new, '--', f]).decode()
    lab = {}
    for h in re.finditer(r'^@@ -\d+(?:,\d+)? \+(\d+)(?:,(\d+))? @@', d, re.M):
        s, n = int(h.group(1)), int(h.group(2) or 1)
        for i in range(max(s, 1), s + max(n, 1)):
            if i - 1 < len(where):
                w = where[i - 1]
                lab[w] = lab.get(w, 0) + 1
    tl = {k: v for k, v in lab.items()}
    print(f[len('specification/'):], tl)
