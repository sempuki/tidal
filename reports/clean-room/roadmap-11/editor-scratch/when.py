import subprocess, sys, json, re
sys.path.insert(0,'/home/ryanm/Code/tidal/reports/clean-room/roadmap-11/editor-scratch')
from count import BLOCK
commits = subprocess.check_output(['git','-C','/home/ryanm/Code/tidal','rev-list','--reverse','c67b349b..3bd790ae']).decode().split()
import io, tarfile, os
def blk(c):
    out={}
    files=subprocess.check_output(['git','-C','/home/ryanm/Code/tidal','ls-tree','-r','--name-only',c,'specification']).decode().split()
    for f in files:
        if not f.endswith('.md'): continue
        t=subprocess.check_output(['git','-C','/home/ryanm/Code/tidal','show',f'{c}:{f}']).decode()
        for m in BLOCK.finditer(t):
            body=m.group(3)
            out[m.group(2)]='\n'.join(l for l in body.splitlines() if not re.match(r'^:(tests|rules|exceptions):',l)).strip()
    return out
prev=blk('c67b349b'); res={}
for c in commits:
    cur=blk(c)
    ch=[k for k in cur if prev.get(k)!=cur[k]]
    if ch: res[c[:8]]=ch
    prev=cur
json.dump(res,open('when.json','w'),indent=0)
for c,v in res.items(): print(c, len(v), ' '.join(sorted(v)))
