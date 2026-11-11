import re,glob,sys
# Usage: programs_no_class.py ROOT. Programs under tests/programs/ (one file each, or a
# directory) whose text has no class, template, try block, or #include: the programs that
# an impact diff could hold before build step 4.
root=sys.argv[1]
units={}
for f in glob.glob(root+'/tests/programs/**/*.*',recursive=True):
    rel=f[len(root)+len('/tests/programs/'):]
    parts=rel.split('/')
    key='/'.join(parts[:-1]) if len(parts)>2 else rel
    units.setdefault(key,[]).append(f)
ok=[]
for k,fs in units.items():
    t=''.join(open(f,errors='replace').read() for f in fs)
    t=re.sub(r'//.*','',t)
    if re.search(r'\b(struct|class|template|try|union)\b',t) or '#include' in t: continue
    ok.append(k)
print(len(units),'program units;',len(ok),'with no class, template, try, union, or include:',' '.join(sorted(ok)))
