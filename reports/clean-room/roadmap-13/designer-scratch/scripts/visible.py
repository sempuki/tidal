import re,glob,sys
# Usage: visible.py ROOT. reject rules cited by an accept or run test whose text holds `requires`.
root=sys.argv[1]
rej=set()
for p in glob.glob(root+'/specification/**/*.md',recursive=True):
    for m in re.finditer(r':::\{rule\} (\S+)\n:kind: reject',open(p).read()): rej.add(m.group(1))
hit=set()
for f in glob.glob(root+'/tests/**/*.cpp',recursive=True):
    t=open(f,errors='replace').read()
    e=re.search(r'^// tidal-expect: (\S+)',t,re.M)
    r=re.search(r'^// tidal-rules: (.*)$',t,re.M)
    if not e or not r or e.group(1) not in ('accept','run'): continue
    if 'requires' not in t: continue
    for x in re.split(r'[ ,]+',r.group(1).strip()):
        if x in rej: hit.add(x)
print(len(hit),'of',len(rej))
