import re,glob,collections
cnt=collections.defaultdict(list)
for r in sorted(glob.glob('reports/clean-room/X*/report.md'),key=lambda s:int(s.split('/')[2][1:])):
    x=r.split('/')[2]
    for line in open(r):
        m=re.match(r'\*\*(X\d+-\d+)\*\* \(([^;)]*)[;)]([^)]*)\)',line)
        if not m: continue
        kind=m.group(2).strip()
        ids=set(re.findall(r'\b[A-Z]+-P\d+\b',m.group(3)))
        for i in ids: cnt[i].append((m.group(1),kind))
for k,v in sorted(cnt.items(),key=lambda kv:-len(kv[1])):
    print(len(v),k,' '.join(f'{a}({b})' for a,b in v))
