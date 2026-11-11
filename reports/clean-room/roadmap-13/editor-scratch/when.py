"""Which commit changed which rule or principle text, over a commit range.
Usage: when.py BASE HEAD"""
import subprocess, sys, re
sys.path.insert(0, '/tmp/claude-1000/-home-ryanm-Code-tidal/34cd0ee5-eea0-4b25-b33b-c46b2846603b/scratchpad/w2/reports/clean-room/roadmap-13/editor-scratch')
from count import BLOCK
G = ['git', '-C', '/tmp/claude-1000/-home-ryanm-Code-tidal/34cd0ee5-eea0-4b25-b33b-c46b2846603b/scratchpad/w2']
def blk(c):
    out = {}
    for f in subprocess.check_output(G + ['ls-tree', '-r', '--name-only', c, 'specification']).decode().split():
        if not f.endswith('.md'):
            continue
        t = subprocess.check_output(G + ['show', f'{c}:{f}']).decode()
        for m in BLOCK.finditer(t):
            out[m.group(2)] = '\n'.join(l for l in m.group(3).splitlines() if not re.match(r'^:(tests|rules|exceptions):', l)).strip()
    return out
base, head = sys.argv[1], sys.argv[2]
prev = blk(base)
for c in subprocess.check_output(G + ['rev-list', '--reverse', f'{base}..{head}']).decode().split():
    cur = blk(c)
    ch = sorted(k for k in cur if prev.get(k) != cur[k])
    if ch:
        print(c[:8], len(ch), ' '.join(ch))
    prev = cur
