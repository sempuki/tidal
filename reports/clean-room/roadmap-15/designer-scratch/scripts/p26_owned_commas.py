import sys,collections
# Usage: p26_owned_commas.py ROOT. Joins X26's scan of tests/ (scan_tests.out) with
# takeover.tsv: for each kind of comma hit (builtin ok, builtin bad, unevaluated, fold,
# operator, decl, overloaded), the files with hits and the files the new front end owns.
root=sys.argv[1]
owned={l.split('\t')[0] for l in open(root+'/reports/clean-room/rust-frontend/takeover.tsv') if l.strip() and not l.startswith('#')}
files=collections.defaultdict(set)
for l in open(root+'/reports/clean-room/X26/scan_tests.out'):
    p=l.split()
    if len(p)<3: continue
    f=p[0].split(':')[0]; k=p[1]+' '+p[2].split(':')[0]
    if not f.startswith('programs/'): files[k].add(f)
for k in sorted(files):
    o=sorted(files[k]&owned)
    print(k,'files',len(files[k]),'owned',len(o))
    if k.startswith('builtin bad'): print('  owned:',' '.join(o))
