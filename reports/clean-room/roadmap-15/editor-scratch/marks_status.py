"""Status in conformance.tsv, at two commits, of each test of the marks
file at the later commit. Usage: marks_status.py OLD NEW (repository root)."""
import subprocess, sys
def show(c, f): return subprocess.check_output(['git', 'show', f'{c}:{f}']).decode()
def st(c):
    r = {}
    for l in show(c, 'reports/clean-room/rust-frontend/conformance.tsv').splitlines():
        if l.startswith('#') or '\t' not in l: continue
        s, t = l.split('\t')[:2]; r[t] = s
    return r
def marks(c):
    return [l.split('\t')[0].strip() for l in show(c, 'reports/clean-room/rust-frontend/rewrite-marks.tsv').splitlines() if '.cpp' in l and not l.startswith('#')]
a, b = st(sys.argv[1]), st(sys.argv[2])
ma, mb = marks(sys.argv[1]), marks(sys.argv[2])
print('marks', len(ma), '->', len(mb), 'added', sorted(set(mb) - set(ma)), 'removed', sorted(set(ma) - set(mb)))
for t in mb:
    if a.get(t) != b.get(t): print('  changed', t, a.get(t), '->', b.get(t))
print('marked tests passing at new', [t for t in mb if b.get(t) in ('pass', 'PASS', 'pass-check')])
import collections; print(collections.Counter(b.get(t) for t in mb))
