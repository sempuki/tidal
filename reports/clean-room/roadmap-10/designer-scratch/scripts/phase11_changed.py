import re,glob,os,sys
S=sys.argv[1]
def blocks(root):
    d={}
    for p in glob.glob(root+'/specification/**/*.md',recursive=True):
        t=open(p).read()
        for m in re.finditer(r':::\{(rule|principle)\} (\S+)\n(.*?)\n:::',t,re.S):
            body=re.sub(r'^:tests:.*$','',m.group(3),flags=re.M)
            d[m.group(2)]=body
    return d
a=blocks(S); b=blocks('/home/ryanm/Code/tidal')
ch={k for k in b if k in a and a[k]!=b[k]}
print('changed in phase 11:',len(ch),'rules',sum(1 for k in ch if '-P' not in k),'principles',sum(1 for k in ch if '-P' in k))
print(' '.join(sorted(ch)))
os.chdir('/home/ryanm/Code/tidal')
tot=0;reps=0
for r in sorted(glob.glob('reports/clean-room/X*/report.md'),key=lambda s:int(s.split('/')[2][1:])):
    t=open(r).read()
    rows=set(re.findall(r'^\| ([A-Z]+-P?\d+) \|',t,re.M))
    hit=sorted(rows&ch)
    if hit: print(r.split('/')[2],len(hit),' '.join(hit)); tot+=len(hit); reps+=1
print('rows',tot,'reports',reps)
