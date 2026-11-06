import re,glob,collections,sys,subprocess
rules={}
for f in glob.glob('specification/**/*.md',recursive=True):
    txt=open(f).read()
    for m in re.finditer(r'^:::\{rule\} ([A-Z]+-\d+)\n((?::[a-z]+:.*\n)*)',txt,re.M):
        rid=m.group(1); hdr=m.group(2)
        c=re.search(r':kind: *(\S+)',hdr)
        rules[rid]=(c.group(1) if c else '?',f)
print(len(rules),collections.Counter(v[0] for v in rules.values()))
src=''
for f in glob.glob('frontend/src/**/*.rs',recursive=True): src+=open(f).read()
for kind in ['reject','unspecified']:
    ids=[r for r,v in rules.items() if v[0]==kind]
    absent=[r for r in ids if not re.search(r'\b'+re.escape(r)+r'\b',src)]
    print(kind,len(ids),'absent',len(absent))
    if kind=='unspecified': print(sorted(absent)); print(sorted(set(ids)-set(absent)))
