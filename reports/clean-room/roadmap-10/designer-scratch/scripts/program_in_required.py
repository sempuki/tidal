import re,glob
hits=[]
for p in glob.glob('specification/**/*.md',recursive=True):
    t=open(p).read()
    for m in re.finditer(r':::\{rule\} (\S+)\n:kind: required\n(.*?)\n:::',t,re.S):
        body=re.sub(r'^:tests:.*$','',m.group(2),flags=re.M)
        if re.search(r'\b(the|a) program\s+must\b',body,re.I):
            hits.append(m.group(1))
print(len(hits),' '.join(sorted(hits)))
