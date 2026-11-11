#!/usr/bin/env python3
"""Writes a probe with the usual prelude and runs tidal-fe on it.
Usage: gen.py NAME DECLS BODY (BODY declares int r)."""
import subprocess, sys
name, decls, body = sys.argv[1:4]
src = ('namespace std { using size_t = decltype(sizeof 0); }\n'
       'inline void* operator new(std::size_t, void* p) noexcept { return p; }\n'
       f'{decls}\nint main() {{\n{body}\n  return r;\n}}\n')
open(name + '.cpp', 'w').write(src)
fe = '/tmp/claude-1000/-home-ryanm-Code-tidal/34cd0ee5-eea0-4b25-b33b-c46b2846603b/scratchpad/target/release/tidal-fe'
out = subprocess.run([fe, 'run', name + '.cpp'], capture_output=True, text=True, timeout=60)
print(name + ':', (out.stdout + out.stderr).splitlines()[0][:200])
