import sys,collections
PASS={'pass','EDG-ONLY','EDG-ALLOWED','pass-partial'}
def load(p):
    d={}
    for l in open(p):
        if l.startswith('#') or not l.strip(): continue
        s,t=l.rstrip('\n').split('\t')[:2]; d[t]=s
    return d
for p in sys.argv[1:]:
    d=load(p); tot=collections.Counter(); fail=collections.Counter(); st=collections.defaultdict(collections.Counter)
    ch=collections.Counter(); chf=collections.Counter(); chst=collections.defaultdict(collections.Counter)
    for t,s in d.items():
        k='/'.join(t.split('/')[:2]); c=t.split('/')[0]
        tot[k]+=1; ch[c]+=1
        if s not in PASS: fail[k]+=1; st[k][s]+=1; chf[c]+=1; chst[c][s]+=1
    print('==',p)
    for k in sorted(tot,key=lambda k:-fail[k]/tot[k])[:14]:
        print(f"{k}\t{fail[k]}/{tot[k]}\t{100*fail[k]/tot[k]:.1f}%\t{dict(st[k])}")
    for c in sorted(ch,key=lambda c:-chf[c]/ch[c]):
        print(f"{c}\t{chf[c]}/{ch[c]}\t{100*chf[c]/ch[c]:.1f}%\t{dict(chst[c])}")
