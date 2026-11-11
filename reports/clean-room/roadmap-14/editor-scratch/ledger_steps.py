"""Moves of the exercise and front-end ledgers at each commit of a range.
Usage: ledger_steps.py BASE HEAD (run from the repository root)."""
import subprocess, sys, re, collections
def rows(c, f):
    r = {}
    try: t = subprocess.check_output(['git', 'show', f'{c}:{f}'], stderr=subprocess.DEVNULL).decode()
    except subprocess.CalledProcessError: return r
    for l in t.splitlines():
        if re.match(r'^\| [A-Z]', l) and not l.startswith('| Finding'):
            x = [y.strip() for y in l.strip().strip('|').split('|')]
            if len(x) >= 4: r[x[0]] = x
    return r
cs = subprocess.check_output(['git', 'rev-list', '--reverse', f'{sys.argv[1]}..{sys.argv[2]}']).decode().split()
prev = sys.argv[1]
for c in cs:
    for f in ['reports/clean-room/ledger.md', 'reports/clean-room/rust-frontend/ledger.md']:
        a, b = rows(prev, f), rows(c, f)
        mv = collections.defaultdict(list)
        for k in b:
            s0 = a[k][2] if k in a else 'new'
            if s0 != b[k][2]: mv[(s0, b[k][2])].append(k)
            elif k in a and a[k][3] != b[k][3]: mv[('record', b[k][2])].append(k)
        if mv:
            print(c[:8], f.split('/')[-2], {f'{x}->{y}': (len(v), ' '.join(v) if len(v) < 12 else v[0] + '..' + v[-1]) for (x, y), v in mv.items()})
    prev = c
