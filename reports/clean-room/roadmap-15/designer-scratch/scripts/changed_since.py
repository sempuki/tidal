import re,glob,os,sys
# Usage: changed_since.py OLDROOT NEWROOT. Lists rule and principle blocks whose text
# (without :tests:) differs, new IDs, and the Propose reports (X*) under NEWROOT whose
# change tables have a row for each changed ID, or whose text names it.
old,new=sys.argv[1],sys.argv[2]
def blocks(root):
    d={}
    for p in glob.glob(root+'/specification/**/*.md',recursive=True):
        t=open(p).read()
        for m in re.finditer(r':::\{(rule|principle)\} (\S+)\n(.*?)\n:::',t,re.S):
            d[m.group(2)]=re.sub(r'^:tests:.*$','',m.group(3),flags=re.M)
    return d
a=blocks(old); b=blocks(new)
ch=sorted(k for k in b if k in a and a[k]!=b[k])
print('changed',len(ch),' '.join(ch)); print('new',sorted(k for k in b if k not in a))
nrows=0; reps=set()
for k in ch:
    rows=[];ment=[]
    for r in sorted(glob.glob(new+'/reports/clean-room/X*/report.md'),key=lambda s:int(s.split('/')[-2][1:])):
        t=open(r).read(); x=r.split('/')[-2]
        if re.search(r'^\| '+re.escape(k)+r' \|',t,re.M): rows.append(x)
        elif re.search(r'\b'+re.escape(k)+r'\b',t): ment.append(x)
    nrows+=len(rows); reps|=set(rows)
    print(k,'rows:',','.join(rows) or '-','| mentions:',','.join(ment) or '-')
print('change-table rows',nrows,'in',len(reps),'reports',sorted(reps,key=lambda s:int(s[1:])))
