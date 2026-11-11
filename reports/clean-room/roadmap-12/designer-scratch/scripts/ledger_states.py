import re,collections,sys
# Count the State column of the exercise ledger and the front-end ledger.
root=sys.argv[1] if len(sys.argv)>1 else '.'
c=collections.Counter()
for l in open(root+'/reports/clean-room/ledger.md'):
    m=re.match(r'\| (\S+) \| `[^|]*` \| ([^|]+?) \|',l)
    if m: c[m.group(2)]+=1
print('exercise ledger',sum(c.values()),dict(c))
c=collections.Counter()
for l in open(root+'/reports/clean-room/rust-frontend/ledger.md'):
    m=re.match(r'\| (G\d+) \|.*?\| (open|commit|question|text|out of scope|planned) \|',l)
    if m: c[m.group(2)]+=1
print('front-end ledger',dict(c))
