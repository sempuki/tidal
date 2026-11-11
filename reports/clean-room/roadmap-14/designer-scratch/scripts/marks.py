import sys,collections
# Usage: marks.py ROOT. Counts the rows of the marks file by model section.
root=sys.argv[1]
c=collections.Counter()
for l in open(root+'/reports/clean-room/rust-frontend/rewrite-marks.tsv'):
    if l.startswith('#') or not l.strip(): continue
    c[l.split('\t')[1]]+=1
print('marked',sum(c.values()),dict(sorted(c.items())))
