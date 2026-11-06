import sys, json, collections
sys.path.insert(0, '.')
from count import blocks, tests
a, b = blocks('t7'), blocks('t8')
ch = [k for k in b if k in a and a[k]['text'] != b[k]['text']]
new = [k for k in b if k not in a]
kindch = [(k, a[k]['kind'], b[k]['kind']) for k in b if k in a and a[k]['kind'] != b[k]['kind']]
rules = sorted(k for k in ch if b[k]['type'] == 'rule')
prin = sorted(k for k in ch if b[k]['type'] == 'principle')
print('changed rules', len(rules), 'principles', len(prin), 'new', new, 'kindch', kindch)
bych = collections.Counter(b[k]['page'].split('/')[0] for k in ch)
bypage = collections.Counter(b[k]['page'] for k in ch)
print(bych.most_common()); print(bypage.most_common(20))
print('RULES', ' '.join(rules)); print('PRIN', ' '.join(prin))
ta, tb = tests('t7'), tests('t8')
newt = sorted(set(tb) - set(ta)); gone = sorted(set(ta) - set(tb))
print('newtests', len(newt), 'gone', len(gone), gone)
ovch = [t for t in tb if t in ta and {k:v for k,v in ta[t].items() if k.startswith('tidal-expect')} != {k:v for k,v in tb[t].items() if k.startswith('tidal-expect')}]
print('expect changed on old tests', len(ovch), ovch[:20])
json.dump(dict(rules=rules, prin=prin, newtests=newt), open('diff.json','w'))
