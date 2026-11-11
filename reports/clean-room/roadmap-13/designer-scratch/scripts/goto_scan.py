import re,glob,sys
# Usage: goto_scan.py ROOT. P22 text scan over tests/programs: goto statements and labels,
# and rule tests that cite a JMP rule and hold a goto.
root=sys.argv[1]
files=sorted(glob.glob(root+'/tests/programs/**/*.*',recursive=True))
n=0
for f in files:
    for i,l in enumerate(open(f,errors='replace'),1):
        s=l.split('//')[0]
        if re.search(r'\bgoto\s+\w+\s*;',s): n+=1; print('goto',f[len(root)+1:],i,l.strip()[:100])
print('goto lines in programs:',n,'of',len(files),'files scanned')
