import re,glob,os,sys,collections
# Usage: p22_reach.py ROOT. For the tests under tests/statements/{jumps,loops,blocks,switch,range-for},
# the chapters of the rules their tidal-rules lines cite, and how many tests cite rules only
# from chapters that build step 3 writes (foundations, declarations, expressions, statements).
root=sys.argv[1]
chap={}
for p in glob.glob(root+'/specification/**/*.md',recursive=True):
    c=p.split('/specification/')[1].split('/')[0]
    for m in re.finditer(r':::\{(?:rule|principle)\} (\S+)',open(p).read()): chap[m.group(1)]=c
step3={'foundations','declarations','expressions','statements'}
tot=0; only3=0; by=collections.Counter(); later=collections.Counter()
for d in ['jumps','loops','blocks','switch','range-for']:
    for f in sorted(glob.glob(root+'/tests/statements/%s/*.cpp'%d)):
        t=open(f,errors='replace').read(3000)
        r=re.search(r'^// tidal-rules: (.*)$',t,re.M)
        if not r: continue
        ids=[x for x in re.split(r'[ ,]+',r.group(1).strip()) if x]
        cs={chap.get(x,'?') for x in ids}
        tot+=1
        for c in cs: by[c]+=1
        if cs<=step3: only3+=1
        else: later[d]+=1
print('tests',tot,'cite step-3 chapters only',only3)
print('tests citing each chapter',dict(sorted(by.items())))
print('tests that cite a later chapter, by directory',dict(later))
# Second count: the same tests by what their text uses (a class with a destructor or
# constructor, a template, a try block), which a page of step 4 or 5 checks.
use=collections.Counter(); none=0
for d in ['jumps','loops','blocks','switch','range-for']:
    for f in sorted(glob.glob(root+'/tests/statements/%s/*.cpp'%d)):
        t=open(f,errors='replace').read()
        if not re.search(r'^// tidal-expect:',t,re.M): continue
        k=[]
        if re.search(r'~\w+\s*\(',t) or re.search(r'\b(struct|class)\s+\w+\s*[{:]',t): k.append('class')
        if re.search(r'\btemplate\s*<',t): k.append('template')
        if re.search(r'\btry\b',t): k.append('try')
        for x in k: use[x]+=1
        if not k: none+=1
print('tests by use',dict(use),'tests with none of these',none)
