"""Statuses in conformance.tsv of the tests that a front-end report's
table of unmarked failures names, and the unmarked failing tests the
table leaves out. Usage: fe_table.py REPORT (run from the repository root)."""
import collections, sys, re
st = {}
for l in open('reports/clean-room/rust-frontend/conformance.tsv'):
    if l.startswith('#'): continue
    s, t = l.rstrip('\n').split('\t')[:2]; st[t] = s
rows = [l.split('`')[1] for l in open(sys.argv[1]) if l.startswith('| `') and l.count('|') >= 5]
marked = {l.split('\t')[0] for l in open('reports/clean-room/rust-frontend/rewrite-marks.tsv') if '.cpp' in l and not l.startswith('#')}
fail = {t for t, s in st.items() if s in ('MISSED-REJECT', 'FALSE-REJECT', 'EDG-AGREES', 'unsupported')}
tab = [t for t in rows if t in fail - marked]
print('table rows', len(tab), collections.Counter(st[t] for t in tab))
print('unmarked failing not in the table', sorted(fail - marked - set(tab)))
