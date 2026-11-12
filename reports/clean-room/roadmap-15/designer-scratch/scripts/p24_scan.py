import re,glob,sys
# Usage: p24_scan.py ROOT. P24 text scan. For tests/ (programs apart), the files that declare a
# scoped enumeration, and the files where an operator | & ^ ~ (or a compound form) takes a
# parameter whose type names a scoped enumeration of the same file.
root=sys.argv[1]
def scan(files):
    se=0; hits=[]
    for f in files:
        t=open(f,errors='replace').read()
        names=re.findall(r'\benum\s+(?:class|struct)\s+(\w+)',t)
        if not names: continue
        se+=1
        for m in re.finditer(r'operator\s*(\|=|&=|\^=|\||&|\^|~)\s*\(([^)]*)\)',t):
            if any(re.search(r'\b'+n+r'\b',m.group(2)) for n in names):
                hits.append((f[len(root)+1:],m.group(1))); break
    return se,hits
allf=sorted(glob.glob(root+'/tests/**/*.*',recursive=True))
prog=[f for f in allf if '/tests/programs/' in f]; rest=[f for f in allf if f not in prog and f.endswith('.cpp')]
for name,fs in (('programs',prog),('rule tests',rest)):
    se,h=scan(fs)
    print(name,'files with a scoped enumeration',se,'with a bitwise operator on one',len(h))
    for x in h: print('  ',x[0],x[1])
