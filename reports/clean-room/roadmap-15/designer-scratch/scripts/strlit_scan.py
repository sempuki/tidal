import re,glob,sys
# Usage: strlit_scan.py ROOT. P29 text scan: lines under tests/programs/ with == or != beside a string literal.
root=sys.argv[1]
n=0;files=set()
for f in sorted(glob.glob(root+'/tests/programs/**/*.*',recursive=True)):
    for i,l in enumerate(open(f,errors='replace'),1):
        if re.search(r'[!=]=\s*"|"\s*[!=]=',l):
            n+=1;files.add(f);print(f[len(root)+1:],i,l.strip()[:120])
print(n,'lines in',len(files),'files')
