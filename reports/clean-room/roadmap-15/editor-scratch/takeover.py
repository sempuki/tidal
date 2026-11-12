"""Checks of takeover.tsv against the tree: row form, duplicates, tests
that exist, marked tests, statuses in conformance.tsv, program tests,
and directories. Usage: takeover.py (repository root)."""
import os, collections
rows = [l.rstrip('\n').split('\t') for l in open('reports/clean-room/rust-frontend/takeover.tsv') if l.strip() and not l.startswith('#')]
print('rows', len(rows), 'forms', collections.Counter(len(r) for r in rows))
print('steps', collections.Counter(r[1] for r in rows if len(r) > 1), 'commits', collections.Counter(r[2] for r in rows if len(r) > 2))
t = [r[0] for r in rows]
print('duplicates', [k for k, v in collections.Counter(t).items() if v > 1])
print('missing', [x for x in t if not os.path.exists('tests/' + x)][:5])
marks = {l.split('\t')[0] for l in open('reports/clean-room/rust-frontend/rewrite-marks.tsv') if '.cpp' in l and not l.startswith('#')}
print('owned and marked', sorted(set(t) & marks))
st = {}
for l in open('reports/clean-room/rust-frontend/conformance.tsv'):
    if l.startswith('#') or '\t' not in l: continue
    s, x = l.rstrip('\n').split('\t')[:2]; st[x] = s
print('old status of owned', collections.Counter(st.get(x) for x in t))
print('programs', sum(1 for x in t if x.startswith('programs/')))
print('top dirs', collections.Counter(x.split('/')[0] for x in t).most_common())
print('pages', len({x.rsplit('/', 1)[0] for x in t}))
