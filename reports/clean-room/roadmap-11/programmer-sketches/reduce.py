#!/usr/bin/env python3
"""Line-based reduction. Usage: reduce.py IN OUT PATTERN
Deletes lines (then single lines again) while g++ -std=c++23
-pedantic-errors still builds the file and runs it to exit 0, and
tidal-fe run still prints PATTERN."""
import subprocess, sys, os, tempfile
FE = '/tmp/claude-1000/-home-ryanm-Code-tidal/34cd0ee5-eea0-4b25-b33b-c46b2846603b/scratchpad/r11/release/tidal-fe'
src, out, pat = sys.argv[1], sys.argv[2], sys.argv[3]
lines = open(src).read().split('\n')
d = tempfile.mkdtemp()
def ok(ls):
    f = os.path.join(d, 't.cpp'); open(f, 'w').write('\n'.join(ls))
    if subprocess.run(['g++', '-std=c++23', '-pedantic-errors', f, '-o', d + '/a'], capture_output=True).returncode: return False
    if subprocess.run([d + '/a'], capture_output=True).returncode: return False
    r = subprocess.run(['timeout', '60', FE, 'run', f], capture_output=True, text=True)
    return pat in r.stdout + r.stderr
assert ok(lines), 'start does not reproduce'
chunk = max(1, len(lines) // 4)
while chunk >= 1:
    i = 0
    while i < len(lines):
        t = lines[:i] + lines[i + chunk:]
        if ok(t): lines = t
        else: i += chunk
    chunk //= 2
open(out, 'w').write('\n'.join(lines))
print(len(lines), 'lines')
