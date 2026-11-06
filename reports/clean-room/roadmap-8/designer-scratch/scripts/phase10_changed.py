import re,glob,os,collections
S='/tmp/claude-1000/-home-ryanm-Code-tidal/558bea91-9fbf-4b01-a330-0a1bb4167438/scratchpad/r8-designer/old'
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
print('changed rules/principles in phase 10:',len(ch), 'rules', sum(1 for k in ch if '-P' not in k))
os.chdir('/home/ryanm/Code/tidal')
for r in sorted(glob.glob('reports/clean-room/X*/report.md'),key=lambda s:int(s.split('/')[2][1:])):
    t=open(r).read()
    rows=set(re.findall(r'^\| ([A-Z]+-P?\d+) \|',t,re.M))
    hit=sorted(rows&ch)
    if hit: print(r.split('/')[2],len(hit),' '.join(hit))
