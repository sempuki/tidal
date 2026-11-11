import re,glob,os,sys
# Usage: changed_since.py OLDSPECDIR ; compares rule/principle text (without :tests:) with the current tree,
# and lists Propose reports whose change tables or text name each changed ID.
old=sys.argv[1]
def blocks(root):
    d={}
    for p in glob.glob(root+'/**/*.md',recursive=True):
        t=open(p).read()
        for m in re.finditer(r':::\{(rule|principle)\} (\S+)\n(.*?)\n:::',t,re.S):
            d[m.group(2)]=re.sub(r'^:tests:.*$','',m.group(3),flags=re.M)
    return d
a=blocks(old); b=blocks('/home/ryanm/Code/tidal/specification')
ch=sorted(k for k in b if k in a and a[k]!=b[k])
new=sorted(k for k in b if k not in a)
print('changed',len(ch),' '.join(ch)); print('new',new)
os.chdir('/home/ryanm/Code/tidal')
for k in ch:
    rows=[];ment=[]
    for r in sorted(glob.glob('reports/clean-room/X*/report.md'),key=lambda s:int(s.split('/')[2][1:])):
        t=open(r).read(); x=r.split('/')[2]
        if re.search(r'^\| '+re.escape(k)+r' \|',t,re.M): rows.append(x)
        elif re.search(r'\b'+re.escape(k)+r'\b',t): ment.append(x)
    print(k,'rows:',','.join(rows) or '-','| mentions:',','.join(ment) or '-')
