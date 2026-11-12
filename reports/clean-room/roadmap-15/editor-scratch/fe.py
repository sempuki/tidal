"""Tests that fail their check at the last front-end round, by test
directory, from conformance.tsv, with the rows of the marks file apart.
Usage: fe.py (run from the repository root)."""
import collections
FAIL = {'EDG-AGREES', 'FALSE-REJECT', 'MISSED-REJECT', 'unsupported'}
f = collections.Counter(); m = collections.Counter()
marked = set()
for l in open('reports/clean-room/rust-frontend/rewrite-marks.tsv'):
    if l.startswith('#') or '.cpp' not in l: continue
    t = l.split('\t')[0].strip(); marked.add(t); m[t.rsplit('/', 1)[0]] += 1
for l in open('reports/clean-room/rust-frontend/conformance.tsv'):
    if l.startswith('#'): continue
    s, t = l.rstrip('\n').split('\t')[:2]
    if s in FAIL: f[t.rsplit('/', 1)[0]] += 1
print('total failing', sum(f.values()), 'marked', len(marked))
for d, n in f.most_common(): print(f'{d:50} {n:3} marked {m[d]}')
