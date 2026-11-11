"""Rule and principle text changed between two trees, and new and changed tests.
Usage: diff.py OLD NEW (each a git archive copy)."""
import sys, collections
sys.path.insert(0, '/tmp/claude-1000/-home-ryanm-Code-tidal/34cd0ee5-eea0-4b25-b33b-c46b2846603b/scratchpad/w2/reports/clean-room/roadmap-13/editor-scratch')
from count import blocks, tests
a, b = blocks(sys.argv[1]), blocks(sys.argv[2])
ch = [k for k in b if k in a and a[k]['text'] != b[k]['text']]
new = sorted(k for k in b if k not in a)
kindch = [(k, a[k]['kind'], b[k]['kind']) for k in b if k in a and a[k]['kind'] != b[k]['kind']]
rules = sorted(k for k in ch if b[k]['type'] == 'rule')
prin = sorted(k for k in ch if b[k]['type'] == 'principle')
print('changed rules', len(rules), 'principles', len(prin), 'new', new, 'kind changes', kindch)
print(collections.Counter(b[k]['page'].split('/')[0] for k in ch + new).most_common())
print(collections.Counter(b[k]['page'] for k in ch + new).most_common(25))
print('RULES', ' '.join(rules)); print('PRIN', ' '.join(prin))
ta, tb = tests(sys.argv[1]), tests(sys.argv[2])
newt = sorted(set(tb) - set(ta)); gone = sorted(set(ta) - set(tb))
print('new tests', len(newt), 'gone', gone)
ex = lambda h: {k: v for k, v in h.items() if k.startswith('tidal-expect')}
print('expectation changed on older tests', [t for t in tb if t in ta and ex(ta[t]) != ex(tb[t])])
