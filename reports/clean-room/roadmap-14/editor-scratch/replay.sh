#!/bin/sh
# Read-only replay of check_specification.py and question_index.py --check
# at each commit of a range, on a copy made with git archive.
# Usage: replay.sh SCRATCH_DIR BASE HEAD
cd /tmp/claude-1000/-home-ryanm-Code-tidal/34cd0ee5-eea0-4b25-b33b-c46b2846603b/scratchpad/rv9c || exit 1
for c in $(git rev-list --reverse "$2..$3"); do
  d="$1/per/$(echo $c | cut -c1-8)"; mkdir -p "$d"
  git archive "$c" | tar -x -C "$d"
  a=$(cd "$d" && python3 -I -B tools/check_specification.py 2>&1 | grep -v '^warning' | tail -3)
  b=$(cd "$d" && python3 -I -B tools/question_index.py --check 2>&1 | tail -1)
  echo "$(echo $c | cut -c1-8) | $a | $b" | tr '\n' ' '; echo
  rm -rf "$d"
done
