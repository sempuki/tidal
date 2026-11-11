"""Word changes of each rule and principle in the RV5 repair commit 3bed9cb5.
Usage: run from a directory that holds pre/ (git archive 3bed9cb5~1 specification)
and post/ (git archive 3bed9cb5 specification)."""
import sys, difflib
sys.path.insert(0, '/home/ryanm/Code/tidal/reports/clean-room/roadmap-10/editor-scratch')
from count import blocks
a, b = blocks('pre'), blocks('post')
for k in sorted(b):
    if k in a and a[k]['text'] != b[k]['text']:
        aw, bw = a[k]['text'].split(), b[k]['text'].split()
        sm = difflib.SequenceMatcher(None, aw, bw)
        ad = sum(j2 - j1 for t, i1, i2, j1, j2 in sm.get_opcodes() if t in ('insert', 'replace'))
        rm = sum(i2 - i1 for t, i1, i2, j1, j2 in sm.get_opcodes() if t in ('delete', 'replace'))
        print(k, b[k]['page'], f'+{ad} -{rm}', len(bw))
