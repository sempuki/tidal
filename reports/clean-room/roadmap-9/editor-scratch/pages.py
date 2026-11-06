import sys, os, re, collections, json
sys.path.insert(0,'.')
from count import blocks, tests, questions, evidence
def table(tree, rep_root):
    B=blocks(tree); T=tests(tree); Q=questions(tree); _,_,ev=evidence(tree)
    bypage=collections.defaultdict(lambda: dict(rules=[],prin=[],tests=set()))
    for k,v in B.items():
        if v['type']=='rule' and v['kind']!='withdrawn':
            bypage[v['page']]['rules'].append(k); bypage[v['page']]['tests'].update(v['tests'])
        elif v['type']=='principle': bypage[v['page']]['prin'].append(k)
    idpage={k:v['page'] for k,v in B.items()}
    F=collections.Counter()
    for d,_,fs in os.walk(rep_root):
        if '/roadmap' in d: continue
        for f in fs:
            if not f.endswith('.md'): continue
            for l in open(os.path.join(d,f),errors='replace'):
                ids=set(re.findall(r'\b[A-Z]+-P?\d+\b',l))
                for p in {idpage[i] for i in ids if i in idpage}: F[p]+=1
    Qo=collections.defaultdict(list)
    for q,v in Q.items():
        if v['status']!='provisional': continue
        ids=set(re.findall(r'\b[A-Z]+-P?\d+\b',v['affects']))
        for p in {idpage[i] for i in ids if i in idpage}: Qo[p].append(q)
    out={}
    for p,d in bypage.items():
        X=sum(1 for t in d['tests'] if t in T and any(k.startswith('tidal-expect-') for k in T[t]))
        out[p]=dict(R=len(d['rules']),T=len(d['tests']),Q=sorted(Qo[p]),F=F[p],X=X,Ev=ev.get(p),RP=round(len(d['rules'])/max(1,len(d['prin'])),1))
    return out
if __name__=='__main__':
    tree=sys.argv[1]
    a=table(tree, os.path.join(tree,'reports','clean-room'))
    json.dump(a,open(f'pages_{tree}.json','w'),indent=0)
    for p,v in sorted(a.items(), key=lambda x:(-len(x[1]['Q']),-x[1]['X']))[:25]:
        print(f"{p:45} {v['R']:3}/{v['T']:3} Q{len(v['Q']):2} F{v['F']:4} X{v['X']:3} Ev{v['Ev'][0]:4}/{v['Ev'][1]:5} RP{v['RP']} {' '.join(v['Q'])}")
