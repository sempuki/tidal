"""New and changed .cpp files under tests/ in each commit, with the
outcome in each new test's tidal-expect line and whether it has a
per-compiler override. Usage: newtests.py COMMIT... (repository root)."""
import subprocess, sys, re, collections
for c in sys.argv[1:]:
    out = subprocess.check_output(['git', 'show', '--format=', '--name-status', c, '--', 'tests']).decode().split('\n')
    new = [l.split('\t')[1] for l in out if l.startswith('A') and l.endswith('.cpp')]
    mod = [l.split('\t')[1] for l in out if l.startswith('M') and l.endswith('.cpp')]
    k = collections.Counter(); ov = 0; prog = 0; nohdr = []
    for f in new:
        t = subprocess.check_output(['git', 'show', f'{c}:{f}']).decode(errors='replace')
        m = re.search(r'^// tidal-expect: *(\S+)', t, re.M)
        k[m.group(1) if m else 'none'] += 1
        if not m: nohdr.append(f)
        if re.search(r'^// tidal-expect-', t, re.M): ov += 1
        if '/programs/' in f: prog += 1
    print(c, 'new', len(new), dict(k), 'override', ov, 'programs', prog, 'changed', len(mod))
    if nohdr: print('   no expect', nohdr[:5])
