#!/bin/sh
# Read-only replay of check_specification.py and question_index.py --check
# at each phase 13 commit, on a copy made with git archive.
# Usage: replay.sh SCRATCH_DIR
cd /home/ryanm/Code/tidal || exit 1
for c in $(git rev-list --reverse c67b349b..3bd790ae); do
  d="$1/per/$(echo $c | cut -c1-8)"; mkdir -p "$d"
  git archive "$c" | tar -x -C "$d"
  a=$(cd "$d" && python3 -I tools/check_specification.py 2>&1 | grep -v '^warning')
  b=$(cd "$d" && python3 -I tools/question_index.py --check 2>&1 | tail -1)
  echo "$(echo $c | cut -c1-8) | $a | $b" | tr '\n' ' '; echo
  rm -rf "$d"
done
