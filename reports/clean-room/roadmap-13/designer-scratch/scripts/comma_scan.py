import re,glob,sys
# Usage: comma_scan.py ROOT. P26 text scan over tests/programs: files that declare
# operator, ; folds over the comma; for heads whose increment holds a comma.
root=sys.argv[1]
files=sorted(glob.glob(root+'/tests/programs/**/*.*',recursive=True))
decl=[f for f in files if re.search(r'operator\s*,\s*\(',open(f,errors='replace').read())]
print('declare operator,:',len(decl),decl)
folds=set()
for f in files:
    for i,l in enumerate(open(f,errors='replace'),1):
        if l.lstrip().startswith('//'): continue
        if re.search(r',\s*\.\.\.\s*\)',l): print('fold',f[len(root)+1:],i,l.strip()); folds.add(f)
        m=re.search(r'\bfor\s*\(([^;]*);([^;]*);([^)]*,[^)]*)\)',l)
        if m: print('for-inc',f[len(root)+1:],i,m.group(3).strip())
print('files with a fold over the comma:',len(folds))
