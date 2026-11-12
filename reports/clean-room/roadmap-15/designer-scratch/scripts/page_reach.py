import re,glob,os,sys,collections
# Usage: page_reach.py ROOT PAGE... For each page (path under specification/), the Propose
# reports (X*) that cite one of its rule or principle IDs.
os.chdir(sys.argv[1])
idpage={}
for p in glob.glob('specification/**/*.md',recursive=True):
    for m in re.finditer(r':::\{(?:rule|principle)\} (\S+)',open(p).read()): idpage[m.group(1)]=p
reach=collections.defaultdict(set)
for d in glob.glob('reports/clean-room/X*/report.md'):
    x=d.split('/')[2]
    for m in re.finditer(r'\b([A-Z]+-P?\d+)\b',open(d).read()):
        if m.group(1) in idpage: reach[idpage[m.group(1)]].add(x)
for a in sys.argv[2:]:
    p='specification/'+a
    print(a,len(reach[p]),sorted(reach[p],key=lambda s:int(s[1:])))
