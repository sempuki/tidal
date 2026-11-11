"""Divergence lines whose tag (open, settled, compiler defect) disagrees with
the question file's status. Usage: divtags.py TREE"""
import sys, re, os
sys.path.insert(0, '/tmp/claude-1000/-home-ryanm-Code-tidal/34cd0ee5-eea0-4b25-b33b-c46b2846603b/scratchpad/rv9c/reports/clean-room/roadmap-14/editor-scratch')
from count import questions, pages
tree = sys.argv[1]; Q = questions(tree); bad = []
for p in pages(tree):
    for m in re.finditer(r'\{question\}`(Q-\d{4})` \(([^)]*)\)', open(p).read()):
        q, inner = m.groups()
        if ';' not in inner: continue
        tag = inner.split(';')[-1].strip(); st = Q[q]['status']
        if (tag == 'open') != (st == 'provisional') or tag.startswith('compiler') != st.startswith('compiler'):
            bad.append((os.path.relpath(p, tree + '/specification')[:-3], q, tag.replace('\n', ' '), st))
print(tree, len(bad)); [print('  ', b) for b in bad]
