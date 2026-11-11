import re,glob,sys,difflib
# Usage: block_diff.py OLDROOT NEWROOT ID... Prints a word diff of each block (without :tests:).
old,new,ids=sys.argv[1],sys.argv[2],sys.argv[3:]
def blocks(root):
    d={}
    for p in glob.glob(root+'/specification/**/*.md',recursive=True):
        for m in re.finditer(r':::\{(rule|principle)\} (\S+)\n(.*?)\n:::',open(p).read(),re.S):
            d[m.group(2)]=re.sub(r'^:tests:.*$','',m.group(3),flags=re.M)
    return d
a=blocks(old); b=blocks(new)
for k in ids:
    print('=====',k)
    for l in difflib.unified_diff(a[k].split('. '),b[k].split('. '),lineterm='',n=0):
        if l.startswith(('+','-')) and not l.startswith(('+++','---')): print(l[:400])
