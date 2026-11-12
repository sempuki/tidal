import subprocess, re, sys, collections
REPO='.'  # run from the repository root
def files(c):
    out=subprocess.check_output(['git','-C',REPO,'ls-tree','-r','--name-only',c,'specification']).decode().split()
    return [f for f in out if f.endswith('.md')]
def show(c,f):
    return subprocess.check_output(['git','-C',REPO,'show',f'{c}:{f}']).decode()
def blocks(c):
    d={}
    for f in files(c):
        t=show(c,f)
        for m in re.finditer(r'^:::\{(rule|principle)\} (\S+)\n(.*?)^:::\s*$',t,re.S|re.M):
            body=[l for l in m.group(3).split('\n') if not l.startswith(':tests:') and not l.startswith(':') ]
            d[m.group(2)]=(m.group(1),f,'\n'.join(body).strip(), m.group(3))
    return d
a=blocks(sys.argv[1]); b=blocks(sys.argv[2])
ch=[k for k in b if k in a and a[k][2]!=b[k][2]]
new=[k for k in b if k not in a]
cls=[k for k in b if k in a and re.search(r':kind: (\S+)',a[k][3]) and re.search(r':kind: (\S+)',b[k][3]) and re.search(r':kind: (\S+)',a[k][3]).group(1)!=re.search(r':kind: (\S+)',b[k][3]).group(1)]
r=[k for k in ch if b[k][0]=='rule']; p=[k for k in ch if b[k][0]=='principle']
pages=collections.Counter(b[k][1] for k in ch)
print('rules',len(r),'principles',len(p),'pages',len(pages),'new',new,'classchange',cls)
print(' '.join(sorted(r)))
print(' '.join(sorted(p)))
for f,n in pages.most_common(): print(n,f)
