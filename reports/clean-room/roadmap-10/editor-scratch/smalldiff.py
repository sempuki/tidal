import sys, difflib, json
sys.path.insert(0,'/home/ryanm/Code/tidal/reports/clean-room/roadmap-10/editor-scratch')
from count import blocks
a,b=blocks('t9'),blocks('t10')
d=json.load(open('diff.json'))
for k in d['rules']+d['prin']:
    aw=a[k]['text'].split(); bw=b[k]['text'].split()
    sm=difflib.SequenceMatcher(None,aw,bw)
    added=sum(j2-j1 for t,i1,i2,j1,j2 in sm.get_opcodes() if t in('insert','replace'))
    removed=sum(i2-i1 for t,i1,i2,j1,j2 in sm.get_opcodes() if t in('delete','replace'))
    snippet=' | '.join(' '.join(bw[j1:j2])[:90] for t,i1,i2,j1,j2 in sm.get_opcodes() if t in('insert','replace'))[:200]
    print(f"{k:10} {b[k]['page']:40} +{added:4} -{removed:4}  {snippet}")
