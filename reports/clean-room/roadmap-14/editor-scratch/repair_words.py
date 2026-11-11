"""Word changes of each rule and principle between two specification copies.
Usage: repair_words.py PRE POST (each a directory that holds specification/)."""
import sys, difflib
sys.path.insert(0, '/tmp/claude-1000/-home-ryanm-Code-tidal/34cd0ee5-eea0-4b25-b33b-c46b2846603b/scratchpad/rv9c/reports/clean-room/roadmap-14/editor-scratch')
from count import blocks
a, b = blocks(sys.argv[1]), blocks(sys.argv[2])
for k in sorted(b):
    if k in a and a[k]['text'] != b[k]['text']:
        aw, bw = a[k]['text'].split(), b[k]['text'].split()
        sm = difflib.SequenceMatcher(None, aw, bw)
        ad = sum(j2 - j1 for t, i1, i2, j1, j2 in sm.get_opcodes() if t in ('insert', 'replace'))
        rm = sum(i2 - i1 for t, i1, i2, j1, j2 in sm.get_opcodes() if t in ('delete', 'replace'))
        print(k, b[k]['page'], f'+{ad} -{rm}', len(bw))
