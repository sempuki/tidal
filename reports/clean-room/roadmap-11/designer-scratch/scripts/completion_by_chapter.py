import re,sys,collections
# Sum the page table of section 9 of a front-end round report by chapter.
t=open(sys.argv[1]).read().split('| Page | Failing tests |')[1]
c=collections.Counter()
for l in t.splitlines()[2:]:
    m=re.match(r'\| (\S+?)/\S* \| (\d+) \|',l)
    if not m: break
    c[m.group(1)]+=int(m.group(2))
print(sum(c.values()),dict(c))
