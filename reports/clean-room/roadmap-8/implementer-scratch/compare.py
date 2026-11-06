import sys,collections
def load(p):
    d={}
    for l in open(p):
        if l.startswith('#') or not l.strip(): continue
        s,t=l.rstrip('\n').split('\t')[:2]; d[t]=s
    return d
old=load(sys.argv[1]); new=load(sys.argv[2])
PASS={'pass','EDG-ONLY','EDG-ALLOWED','pass-partial'}
print('old',len(old),collections.Counter(old.values()))
print('new',len(new),collections.Counter(new.values()))
both=collections.Counter(); 
for t,s in new.items():
    if s in PASS: continue
    o=old.get(t)
    both[(o if o is None or o not in PASS else 'PASS', s)]+=1
for k,v in sorted(both.items(),key=lambda x:-x[1]): print(v,k)
fixed=sum(1 for t,s in new.items() if s in PASS and t in old and old[t] not in PASS)
print('fixed',fixed)
worse=[t for t,s in new.items() if s not in PASS and old.get(t) in PASS]; print('worse',worse)
