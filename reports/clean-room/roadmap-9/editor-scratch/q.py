import sys, re, collections, os
sys.path.insert(0,'.')
from count import questions
a=questions('t8'); r=a; b=questions('t9')
new=sorted(set(b)-set(a))
print('new',new,[b[q]['status'] for q in new])
op=lambda Q:{q for q,v in Q.items() if v['status']=='provisional'}
oa,ob=op(a),op(b)
print('settled from open',sorted(oa-ob)); print('reopened',sorted((ob-oa)-set(new)))
strip=lambda t:re.sub(r'^\*\*Waits for:\*\*.*\n','',t,flags=re.M)
untouched=sorted(q for q in ob&oa if strip(b[q]['text'])==a[q]['text'])
print('open, unchanged apart from Waits for line:',len(untouched))
wch=sorted(q for q in ob&op(r) if r[q]['waits']!=b[q]['waits'])
print('waits changed since R4:',len(wch))
# items
def items(w): return [x.strip() for x in w.split(';')]
def kind(x):
    if x.startswith('the maintainer'): return 'maintainer'
    if x.startswith('a review'): return x
    if x.startswith('newest'): return 'newest'
    if x.startswith('a use'): return 'use'
    if x.startswith('a first'): return 'first'
    return x.split(' (')[0]
for name,Q in [('start',r),('end',b)]:
    c=collections.Counter()
    for q in op(Q):
        for k in set(kind(x) for x in items(Q[q]['waits'])): c[k]+=1
    print(name, sorted(c.items()))
# first-item group (exclusive)
order=['maintainer','newest','use','first','a review (against)','a review (silent)','a review (doubt)','a review (part)']
g=collections.Counter()
for q in ob:
    ks=[kind(x) for x in items(b[q]['waits'])]
    for o in order:
        if o in ks: g[o]+=1; break
    else: g['other:'+ks[0]]+=1
print('exclusive',g)
# dated notes count: headings or lines starting with dates
stack=collections.Counter()
for q in ob:
    n=len(re.findall(r'^(?:###? .*|\*\*)?.*Kept open',b[q]['text'],re.M))
    stack[q]=n
print('kept-open >=3:',sorted((q,n) for q,n in stack.items() if n>=3))
# use-waiting
for q in sorted(ob):
    if 'a use' in b[q]['waits'] or 'first review' in b[q]['waits'] or 'principle (' in b[q]['waits'] or 'a rule' in b[q]['waits'] or 'child' in b[q]['waits']:
        print(q,b[q]['waits'])
# review-held untouched since t8
rv=[q for q in ob if 'a review' in b[q]['waits']]
print('review-held',len(rv),'unchanged since t8 apart from waits line', sum(1 for q in rv if q in untouched))
