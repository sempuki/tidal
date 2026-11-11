import subprocess, re, sys, collections
REPO='/home/ryanm/Code/tidal'
c=sys.argv[1]
def git(*a): return subprocess.check_output(['git','-C',REPO]+list(a)).decode()
fs=[f for f in git('ls-tree','-r','--name-only',c).split('\n') if f]
spec=[f for f in fs if f.startswith('specification/') and f.endswith('.md') and not f.endswith('index.md') and 'glossary' not in f]
W=E=EX=0; pages=0
for f in spec:
    t=git('show',f'{c}:{f}')
    if ':::{rule}' not in t and ':::{principle}' not in t: continue
    pages+=1
    W+=len(t.split())
    m=re.search(r'^## Evidence\n(.*?)(?=^## |\Z)',t,re.S|re.M)
    if m: E+=len(m.group(1).split())
    EX+=len(re.findall(r'^```cpp',t,re.M))
tests=[f for f in fs if f.startswith('tests/') and f.endswith('.cpp') and not f.endswith('.tu.cpp')]
prog=[f for f in tests if f.startswith('tests/programs/')]
ov=ovl=err=edg=allow=rf=0
fam=collections.Counter()
for f in tests:
    t=git('show',f'{c}:{f}')
    head=t[:6000]
    lines=re.findall(r'^// tidal-expect-(\S+):',head,re.M)
    if lines: ov+=1; ovl+=len(lines)
    if any('edg' in l for l in lines): edg+=1
    if re.search(r'^// tidal-error',head,re.M): err+=1
    nf=set()
    for l in lines:
        if l in ('clang','clang-22','clang-host'): nf.add('Clang')
        if l in ('gcc','gcc-16','gcc-host'): nf.add('GCC')
        if l=='edg': nf.add('EDG')
    if nf: fam[' and '.join(sorted(nf))]+=1
    if 'tidal-allows-edg' in head: allow+=1
    if re.search(r'tidal-interpreter:\s*run-fail',head): rf+=1
print(c,'pages',pages,'words',W,'evidence',E,'examples',EX,'tests',len(tests),'programs',len(prog),'override files',ov,'override lines',ovl,'edg files',edg,'tidal-error',err,'allows-edg',allow,'interp run-fail',rf)
sim=[f for f in fs if f.startswith('reports/similarity/')]
print('newest',sum(fam.values()),dict(fam))
print('similarity files',len(sim))
q=[f for f in fs if re.match(r'questions/Q-\d+',f)]
print('questions',len(q))
