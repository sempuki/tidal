"""Rules whose :tests: line one commit changes, with the entries added.
Usage: tests_rules.py COMMIT (run from the repository root)."""
import sys
sys.path.insert(0, __file__.rsplit('/', 1)[0])
from tests_lines import tl
c = sys.argv[1]
a, b = tl(c + '^'), tl(c)
for k in sorted(b):
    d = b[k] - a.get(k, set())
    if d: print(k, len(d), ' '.join(sorted(d))[:160])
