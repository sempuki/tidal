import re,glob,sys
# P29 rough scan: lines under tests/programs/ with == or != next to a string literal.
root=sys.argv[1] if len(sys.argv)>1 else '.'
n=0;files=set()
for f in sorted(glob.glob(root+'/tests/programs/**/*.*',recursive=True)):
    for i,l in enumerate(open(f,errors='replace'),1):
        if re.search(r'[!=]=\s*"|"\s*[!=]=',l):
            n+=1;files.add(f);print(f,i,l.strip()[:120])
print(n,'lines in',len(files),'files')
