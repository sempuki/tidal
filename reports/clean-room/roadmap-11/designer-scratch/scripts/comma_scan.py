import re,glob,sys
# P26 rough scan over tests/programs: files that declare operator, ; folds over the comma; for-heads whose increment has a comma.
root=sys.argv[1] if len(sys.argv)>1 else '.'
files=sorted(glob.glob(root+'/tests/programs/**/*.*',recursive=True))
decl=[f for f in files if re.search(r'operator\s*,\s*\(',open(f,errors='replace').read())]
print('declare operator,:',len(decl),decl)
for f in files:
    for i,l in enumerate(open(f,errors='replace'),1):
        if l.lstrip().startswith('//'): continue
        if re.search(r',\s*\.\.\.\s*\)',l): print('fold',f,i,l.strip())
        m=re.search(r'\bfor\s*\(([^;]*);([^;]*);([^)]*,[^)]*)\)',l)
        if m: print('for-inc',f,i,m.group(3).strip())
