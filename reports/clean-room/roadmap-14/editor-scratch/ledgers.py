"""States of both ledgers at two commits, the moves between them, and the
commit rows whose Record names no hash. Usage: ledgers.py OLD NEW"""
import subprocess, sys, re, collections
G = ['git', '-C', '/tmp/claude-1000/-home-ryanm-Code-tidal/34cd0ee5-eea0-4b25-b33b-c46b2846603b/scratchpad/rv9c']
def rows(c, f):
    r = {}
    for l in subprocess.check_output(G + ['show', f'{c}:{f}']).decode().splitlines():
        if re.match(r'^\| [A-Z]', l) and not l.startswith('| Finding'):
            x = [y.strip() for y in l.strip().strip('|').split('|')]
            if len(x) >= 4: r[x[0]] = x
    return r
for f in ['reports/clean-room/ledger.md', 'reports/clean-room/rust-frontend/ledger.md']:
    a, b = rows(sys.argv[1], f), rows(sys.argv[2], f)
    print(f, len(a), dict(collections.Counter(v[2] for v in a.values())), '->', len(b), dict(collections.Counter(v[2] for v in b.values())))
    mv = collections.Counter((a[k][2] if k in a else 'new', b[k][2]) for k in b if k not in a or a[k][2] != b[k][2])
    print('  moves', sorted(mv.items()))
    print('  commit rows with no hash', [k for k, v in b.items() if v[2] == 'commit' and not re.search(r'\b[0-9a-f]{7,40}\b', v[3])])
    print('  open', [k for k, v in b.items() if v[2] in ('open', 'planned')])
    for k, v in b.items():
        if v[2] == 'commit':
            for h in re.findall(r'\b([0-9a-f]{7,40})\b', v[3]):
                if subprocess.run(G + ['cat-file', '-e', h + '^{commit}'], capture_output=True).returncode:
                    print('  unknown hash', k, h)
