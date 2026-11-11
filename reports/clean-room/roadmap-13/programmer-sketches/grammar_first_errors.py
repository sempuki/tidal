#!/usr/bin/env python3
"""Lists the reject tests that the front end rejects with TR-12 (a grammar
error) as its first error while the test cites no TR rule. Such a test
passes its check whatever its own rule says. Read-only: runs
`tidal-fe check` on each single-file test whose tidal-expect is reject.
Usage: grammar_first_errors.py REPO_ROOT TIDAL_FE"""
import re, subprocess, sys
from pathlib import Path

root, fe = Path(sys.argv[1]), sys.argv[2]
hits, total = [], 0
for p in sorted((root / 'tests').rglob('*.cpp')):
    if p.name.endswith('.tu.cpp'):
        continue
    text = p.read_text(errors='replace')
    if not re.search(r'^// tidal-expect: reject\s*$', text, re.M):
        continue
    if re.search(r'^// tidal-(sources|units):', text, re.M):
        continue
    rules = re.search(r'^// tidal-rules:(.*)$', text, re.M)
    rules = rules.group(1).split() if rules else []
    total += 1
    r = subprocess.run(['timeout', '30', fe, 'check', str(p)], capture_output=True, text=True)
    errs = [l for l in (r.stdout + r.stderr).splitlines() if ': error [' in l]
    if errs and ': error [TR-12]' in errs[0] and not any(x.startswith('TR-') for x in rules):
        hits.append((str(p.relative_to(root / 'tests')), ' '.join(rules), errs[0].split(': ', 1)[1][:110]))
for h in hits:
    print(' | '.join(h))
print(f'{len(hits)} of {total} single-file reject tests get TR-12 first and cite no TR rule')
