"""Question changes between two trees: new, settled, reopened, Waits for items
at both ends, and the exclusive groups. Usage: q.py OLD NEW"""
import sys, re, collections
sys.path.insert(0, '/tmp/claude-1000/-home-ryanm-Code-tidal/34cd0ee5-eea0-4b25-b33b-c46b2846603b/scratchpad/w2/reports/clean-room/roadmap-13/editor-scratch')
from count import questions
a, b = questions(sys.argv[1]), questions(sys.argv[2])
op = lambda Q: {q for q, v in Q.items() if v['status'] == 'provisional'}
oa, ob = op(a), op(b)
new = sorted(set(b) - set(a))
print('new', [(q, b[q]['status'], b[q]['waits']) for q in new])
print('settled from open', sorted(oa - ob), 'reopened', sorted((ob - oa) - set(new)))
both = oa & ob
same = sorted(q for q in both if a[q]['text'] == b[q]['text'])
print('open at both ends', len(both), 'unchanged', len(same), 'changed', sorted(both - set(same)))
print('waits changed', [(q, a[q]['waits'], '->', b[q]['waits']) for q in sorted(both) if a[q]['waits'] != b[q]['waits']])
def items(w): return [x.strip() for x in w.split(';')]
def kind(x):
    if x.startswith('the maintainer'): return 'maintainer'
    if x.startswith('a review'): return x
    if x.startswith('newest'): return 'newest'
    if x.startswith('a use'): return 'use'
    if x.startswith('a first'): return 'first'
    return x.split(' (')[0]
for name, Q in [('start', a), ('end', b)]:
    c = collections.Counter()
    for q in op(Q):
        for k in set(kind(x) for x in items(Q[q]['waits'])): c[k] += 1
    r4 = sum(1 for q in op(Q) if re.search(r'row 4', Q[q]['waits']))
    plain = sum(1 for q in op(Q) if 'the maintainer (pending.md row 4)' in Q[q]['waits'])
    print(name, sorted(c.items()), 'name row 4:', r4, 'plain row 4 item:', plain)
order = ['maintainer', 'newest', 'use', 'first', 'a review (against)', 'a review (silent)', 'a review (doubt)', 'a review (part)']
for name, Q in [('start', a), ('end', b)]:
    g = collections.defaultdict(list)
    for q in op(Q):
        ks = [kind(x) for x in items(Q[q]['waits'])]
        for o in order:
            if o in ks: g[o].append(q); break
        else: g['other: ' + ks[0]].append(q)
    print(name, {k: len(v) for k, v in g.items()})
    for k in ['a review (against)', 'use', 'first'] + [k for k in g if k.startswith('other')]:
        print('   ', k, sorted(g[k]))
    d = collections.Counter(max(re.findall(r'2026-\d\d-\d\d', Q[q]['text']) or ['none']) for q in g['a review (doubt)'])
    print('    doubt-only by last date', sorted(d.items()))
