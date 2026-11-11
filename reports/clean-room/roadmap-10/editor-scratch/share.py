import sys,re,collections; sys.path.insert(0,'/home/ryanm/Code/tidal/reports/clean-room/roadmap-10/editor-scratch')
from count import questions
strip=lambda t:re.sub(r'^\*\*Waits for:\*\*.*\n','',t,flags=re.M)
for name in ['t9','t10']:
    Q=questions(name); op={q:v for q,v in Q.items() if v['status']=='provisional'}
    m=[q for q,v in op.items() if 'the maintainer' in v['waits']]
    n=[q for q,v in op.items() if 'newest families' in v['waits']]
    s=[q for q,v in op.items() if 'a review (silent)' in v['waits']]
    rows=collections.Counter()
    for q,v in op.items():
        for r in re.findall(r'pending\.md row (\d+[ab]?)',v['waits']): rows[r]+=1
    u=set(m)|set(n)
    print(name,len(op),'maint',len(m),'newest',len(n),'union rows1-9',len(u),f'{100*len(u)/len(op):.0f}%','with silent',len(u|set(s)),f'{100*len(u|set(s))/len(op):.0f}%',sorted(rows.items()))
a=questions('t9');b=questions('t10')
oa={q for q,v in a.items() if v['status']=='provisional'}; ob={q for q,v in b.items() if v['status']=='provisional'}
un=[q for q in oa&ob if strip(a[q]['text'])==strip(b[q]['text'])]
print('open at both, text unchanged apart from waits:',len(un), 'waits line also unchanged:', sum(1 for q in un if a[q]['waits']==b[q]['waits']))
rv=[q for q in ob if 'a review' in b[q]['waits']]
print('review-held',len(rv),'unchanged in phase 11',sum(1 for q in rv if q in un))
dates=collections.Counter()
for q in rv:
    ds=re.findall(r'2026-\d\d-\d\d',b[q]['text']); dates[max(ds) if ds else None]+=1
print('latest date in review-held:',sorted(dates.items(),key=str))
