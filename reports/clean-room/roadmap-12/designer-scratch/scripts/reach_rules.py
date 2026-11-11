import re,glob,os,subprocess,collections
os.chdir('/home/ryanm/Code/tidal')
idpage={}
for p in glob.glob('specification/**/*.md',recursive=True):
    for m in re.finditer(r':::\{rule\} (\S+)',open(p).read()): idpage[m.group(1)]=p
print('ids',len(idpage))
reach=collections.defaultdict(set)
for d in sorted(glob.glob('reports/clean-room/X*/report.md')):
    x=d.split('/')[2]
    t=open(d).read()
    for m in re.finditer(r'\b([A-Z]+-P?\d+)\b',t):
        if m.group(1) in idpage: reach[idpage[m.group(1)]].add(x)
pages=sorted({p for p in glob.glob('specification/**/*.md',recursive=True) if ':::{rule}' in open(p).read()})
print('pages',len(pages),'reached',sum(1 for p in pages if reach[p]))
def added(x):
    return subprocess.run(['git','log','--diff-filter=A','--format=%h','--','reports/clean-room/%s/report.md'%x],capture_output=True,text=True).stdout.split()[-1]
for p in pages:
    xs=reach[p]
    if len(xs)<=2:
        last=max(xs,key=lambda s:int(s[1:]))
        c=added(last)
        n=len(subprocess.run(['git','log','--format=%h',c+'..HEAD','--',p],capture_output=True,text=True).stdout.split())
        print(len(xs),p,sorted(xs,key=lambda s:int(s[1:])),'commits since',last,n)
cnt=collections.Counter(len(reach[p]) for p in pages); print(sorted(cnt.items()))
