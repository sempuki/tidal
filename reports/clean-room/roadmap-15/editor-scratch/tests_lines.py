"""Test entries that each commit adds to and removes from :tests: lines,
by rule. Usage: tests_lines.py COMMIT... (run from the repository root)."""
import subprocess, sys, re
sys.path.insert(0, '/tmp/claude-1000/-home-ryanm-Code-tidal/34cd0ee5-eea0-4b25-b33b-c46b2846603b/scratchpad/fe28c/reports/clean-room/roadmap-15/editor-scratch')
from count import BLOCK
def tl(c):
    out = {}
    for f in subprocess.check_output(['git', 'ls-tree', '-r', '--name-only', c, 'specification']).decode().split():
        if not f.endswith('.md'): continue
        t = subprocess.check_output(['git', 'show', f'{c}:{f}']).decode()
        for m in BLOCK.finditer(t):
            x = re.search(r'^:tests: ?(.*)$', m.group(3), re.M)
            out[m.group(2)] = set(x.group(1).split()) if x else set()
    return out
if __name__ == '__main__':
  for c in sys.argv[1:]:
    a, b = tl(c + '^'), tl(c)
    add = {k: sorted(b[k] - a.get(k, set())) for k in b if b[k] - a.get(k, set())}
    rem = {k: sorted(a[k] - b.get(k, set())) for k in a if a[k] - b.get(k, set())}
    print(c, 'rules gaining', len(add), 'entries added', sum(map(len, add.values())), 'distinct tests', len(set().union(*add.values())) if add else 0, 'removed', rem)
