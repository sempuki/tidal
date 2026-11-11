"""For each block ID given, the review files under reports/review/ dated
2026-11-11 with phase 16 or rv9 in the name that name it.
Usage: named.py ID..."""
import sys, re, glob
fs = sorted(glob.glob('reports/review/2026-11-11-*phase-16*') + glob.glob('reports/review/2026-11-11-read-of-rv9*'))
T = {f.split('/')[-1][11:]: open(f).read() for f in fs}
for i in sys.argv[1:]:
    hit = [k[:40] for k, t in T.items() if re.search(r'\b' + re.escape(i) + r'\b', t)]
    print(f'{i:8}', hit or 'NONE')
