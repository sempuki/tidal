import re,collections,sys
# Usage: ledger_states.py ROOT. Counts the State column of both ledgers and lists open rows.
root=sys.argv[1]
c=collections.Counter(); op=[]
for l in open(root+'/reports/clean-room/ledger.md'):
    m=re.match(r'\| (\S+) \| `[^|]*` \| ([^|]+?) \|',l)
    if m:
        c[m.group(2)]+=1
        if m.group(2)=='open': op.append(m.group(1))
print('exercise ledger',sum(c.values()),dict(c)); print('open',' '.join(op))
c=collections.Counter()
for l in open(root+'/reports/clean-room/rust-frontend/ledger.md'):
    m=re.match(r'\| (G\d+) \|.*?\| (open|commit|question|text|out of scope|planned) \|',l)
    if m: c[m.group(2)]+=1
print('front-end ledger',dict(c))
