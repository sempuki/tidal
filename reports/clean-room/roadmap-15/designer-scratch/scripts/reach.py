import re,glob,os,subprocess,collections,sys
# Usage: reach.py ROOT. For each page with rules, the Propose reports (X*) that cite one of
# its rule or principle IDs; pages reached by at most two reports, with commits to the page
# since the latest of those reports was added.
os.chdir(sys.argv[1])
idpage={}
for p in glob.glob('specification/**/*.md',recursive=True):
    for m in re.finditer(r':::\{(?:rule|principle)\} (\S+)',open(p).read()): idpage[m.group(1)]=p
reach=collections.defaultdict(set)
for d in sorted(glob.glob('reports/clean-room/X*/report.md')):
    x=d.split('/')[2]
    for m in re.finditer(r'\b([A-Z]+-P?\d+)\b',open(d).read()):
        if m.group(1) in idpage: reach[idpage[m.group(1)]].add(x)
pages=sorted({p for p in glob.glob('specification/**/*.md',recursive=True) if ':::{rule}' in open(p).read()})
print('pages',len(pages),'reached',sum(1 for p in pages if reach[p]))
def added(x):
    return subprocess.run(['git','log','--diff-filter=A','--format=%h','--','reports/clean-room/%s/report.md'%x],capture_output=True,text=True).stdout.split()[-1]
for p in pages:
    xs=reach[p]
    if 0<len(xs)<=2:
        last=max(xs,key=lambda s:int(s[1:]))
        n=len(subprocess.run(['git','log','--format=%h',added(last)+'..HEAD','--',p],capture_output=True,text=True).stdout.split())
        print(len(xs),p,sorted(xs,key=lambda s:int(s[1:])),'commits since',last,n)
x25=sorted(p for p in pages if 'X25' in reach[p]); print('X25 reaches',len(x25),' '.join(q.split('specification/')[1] for q in x25))
print(sorted(collections.Counter(len(reach[p]) for p in pages).items()))
