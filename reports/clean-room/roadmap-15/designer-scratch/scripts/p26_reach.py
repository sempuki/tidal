import re,glob,os,sys
# Usage: p26_reach.py ROOT. For the tests that X26's change table names (section 2 and 4.4),
# and for every test under tests/expressions/comma/, prints: expectation, the chapters its
# tidal-rules line cites, what its text uses (class, template, operator function, fold),
# and whether takeover.tsv lists it (the new front end owns it at build step 3).
root=sys.argv[1]
chap={}
for p in glob.glob(root+'/specification/**/*.md',recursive=True):
    c=p.split('/specification/')[1].split('/')[0]
    for m in re.finditer(r':::\{(?:rule|principle)\} (\S+)',open(p).read()): chap[m.group(1)]=c
owned=set()
for l in open(root+'/reports/clean-room/rust-frontend/takeover.tsv'):
    if l.startswith('#') or not l.strip(): continue
    owned.add(l.split('\t')[0])
rep=open(root+'/reports/clean-room/X26/report.md').read()
named=set(re.findall(r'`((?:expressions|classes|overloading|foundations|declarations|templates)/[\w/-]+\.cpp)`',rep))
# short names in the X26:new-1 row and the COMMA rows are relative to expressions/comma/
for s in re.findall(r'`([\w-]+\.cpp)`',rep):
    if os.path.exists(root+'/tests/expressions/comma/'+s): named.add('expressions/comma/'+s)
allc={os.path.relpath(f,root+'/tests') for f in glob.glob(root+'/tests/expressions/comma/*.cpp')}
step3={'foundations','declarations','expressions','statements'}
rows=[]
for t in sorted(named|allc):
    f=root+'/tests/'+t
    if not os.path.exists(f): print('missing',t); continue
    s=open(f,errors='replace').read()
    e=re.search(r'^// tidal-expect: (\S+)',s,re.M); r=re.search(r'^// tidal-rules: (.*)$',s,re.M)
    ids=[x for x in re.split(r'[ ,]+',r.group(1).strip()) if x] if r else []
    cs=sorted({chap.get(x,'?') for x in ids})
    use=[]
    if re.search(r'\b(struct|class|union)\s+\w+',s): use.append('class')
    if re.search(r'\btemplate\s*<',s): use.append('template')
    if re.search(r'\boperator\s*,',s): use.append('operator,')
    if re.search(r'\.\.\.\s*\)',s) and re.search(r',\s*\.\.\.',s): use.append('fold')
    plain = set(cs)<=step3 and not use
    rows.append((t,e.group(1) if e else '-',','.join(cs),','.join(use) or '-', 'X26' if t in named else '', 'owned' if t in owned else '', 'plain-guess' if plain else ''))
for r in rows: print('\t'.join(r))
n=[r for r in rows if r[4]]
print('X26-named tests',len(n),'owned',sum(1 for r in n if r[5]),'no class/template/operator,/fold and step-3 chapters only',sum(1 for r in n if r[6]))
print('comma dir tests',len(allc),'owned',sum(1 for r in rows if r[0] in allc and r[5]))
