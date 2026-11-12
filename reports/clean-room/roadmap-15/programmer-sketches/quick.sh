#!/bin/bash
# Usage: quick.sh SOURCE: g++ and clang++ with the Write flags, then both front ends.
S=/tmp/claude-1000/-home-ryanm-Code-tidal/34cd0ee5-eea0-4b25-b33b-c46b2846603b/scratchpad
B=$(mktemp -d $S/q.XXXX)
for cxx in g++ clang++; do
  if $cxx -std=c++23 -pedantic-errors -Wall -Wextra "$@" -o $B/a 2>$B/err; then
    timeout 20 $B/a; echo "$cxx: exit $? (warnings $(grep -c warning: $B/err))"; grep -m3 warning: $B/err
  else echo "$cxx: FAILED"; grep -m4 error $B/err; fi
done
echo "tidal-fe: $(timeout 120 $S/old/release/tidal-fe run "$@" 2>&1 | head -4)"
echo "tidal-next: $(timeout 120 $S/next/release/tidal-next run "$@" 2>&1 | head -2)"
rm -rf $B
