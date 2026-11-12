import re, sys, os, collections, glob
# Usage: counts.py ROOT. Rule kinds, pages, IDs, and test expectations of a tree.
root = sys.argv[1]
spec = os.path.join(root, 'specification')
kinds = collections.Counter(); ids=set(); pages=set()
for p in glob.glob(spec+'/**/*.md', recursive=True):
    t=open(p).read()
    for m in re.finditer(r':::\{rule\} (\S+)\n:kind: (\S+)', t):
        ids.add(m.group(1)); kinds[m.group(2)]+=1; pages.add(p)
print('pages', len(pages), 'ids', len(ids)); print(dict(sorted(kinds.items())))
tests=glob.glob(root+'/tests/**/*.cpp', recursive=True)
n=rej=lf=m32=0
for f in tests:
    h=open(f,errors='replace').read(4000)
    m=re.search(r'^// tidal-expect: (\S+)', h, re.M)
    if not m: continue
    n+=1
    if m.group(1)=='reject': rej+=1
    if m.group(1)=='link-fail': lf+=1
    if re.search(r'^// tidal-flags:.*-m32', h, re.M): m32+=1
print('tests', n, 'reject', rej, 'link-fail', lf, 'm32', m32)
