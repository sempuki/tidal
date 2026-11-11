import sys,re,os,collections; sys.path.insert(0,'/home/ryanm/Code/tidal/reports/clean-room/roadmap-11/editor-scratch')
from count import questions,pages
for tree in ['t10','t11']:
    Q=questions(tree); bad=[]
    for p in pages(tree):
        t=open(p).read()
        for m in re.finditer(r'\{question\}`(Q-\d{4})` \(([^)]*)\)',t):
            q,inner=m.groups()
            if ';' not in inner: continue
            tag=inner.split(';')[-1].strip()
            st=Q[q]['status']
            if (tag=='open') != (st=='provisional') or (tag.startswith('compiler') != st.startswith('compiler')):
                bad.append((os.path.relpath(p,tree+'/specification')[:-3],q,tag.replace('\n',' '),st))
    print(tree,len(bad))
    for b in bad: print('  ',b)
