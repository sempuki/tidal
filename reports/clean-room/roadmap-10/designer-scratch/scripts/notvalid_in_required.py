import re,glob,sys
# Count `required` rules whose own text says a program or expression "is not valid" or "makes ... not valid".
root=sys.argv[1] if len(sys.argv)>1 else '.'
hits=[]
for p in glob.glob(root+'/specification/**/*.md',recursive=True):
    t=open(p).read()
    for m in re.finditer(r':::\{rule\} (\S+)\n:kind: required\n(.*?)\n:::',t,re.S):
        body=re.sub(r'^:tests:.*$','',m.group(2),flags=re.M)
        if re.search(r'\bnot valid\b',body): hits.append(m.group(1))
print(len(hits),' '.join(sorted(hits)))
