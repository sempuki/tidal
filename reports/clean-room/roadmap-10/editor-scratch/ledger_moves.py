import subprocess,sys,collections
def rows(c):
    t=subprocess.check_output(['git','-C','/home/ryanm/Code/tidal','show',f'{c}:reports/clean-room/ledger.md']).decode()
    r={}
    for l in t.splitlines():
        if l.startswith('| ') and not l.startswith('| Finding'):
            x=[y.strip() for y in l.strip().strip('|').split('|')]
            if len(x)>=4: r[x[0]]=(x[2],x[3])
    return r
a,b=rows(sys.argv[1]),rows(sys.argv[2])
moves=collections.Counter((a[k][0] if k in a else 'new',b[k][0]) for k in b if a.get(k,('',''))[0]!=b[k][0])
print(sorted(moves.items()))
for k in b:
    if a.get(k,('',''))[0]!=b[k][0] and (len(sys.argv)>3): print(' ',k,a.get(k,('-',''))[0],'->',b[k][0],b[k][1][:80])
print(len(a),len(b),collections.Counter(v[0] for v in b.values()))
