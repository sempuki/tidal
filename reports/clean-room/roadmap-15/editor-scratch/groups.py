"""Exclusive groups of open questions by their Waits for line, with the last
date in each question of the doubt and against groups. Usage: groups.py TREE"""
import sys, re, collections
sys.path.insert(0, '/tmp/claude-1000/-home-ryanm-Code-tidal/34cd0ee5-eea0-4b25-b33b-c46b2846603b/scratchpad/fe28c/reports/clean-room/roadmap-15/editor-scratch')
from count import questions
Q = questions(sys.argv[1])
op = {q: v for q, v in Q.items() if v['status'] == 'provisional'}
order = ['the maintainer', 'newest families', 'a use', 'a first review', 'a review (against)', 'a review (silent)', 'a review (doubt)', 'a review (part)']
g = collections.defaultdict(list)
for q, v in op.items():
    for o in order:
        if o in v['waits']:
            g[o].append(q); break
    else:
        g['other'].append(q)
for o, l in g.items():
    print(o, len(l))
for o in ['a review (against)', 'a review (doubt)', 'a review (silent)']:
    d = collections.Counter(max(re.findall(r'2026-\d\d-\d\d', op[q]['text']) or ['none']) for q in g[o])
    print(o, sorted(g[o]) if o != 'a review (doubt)' else '', sorted(d.items()))
