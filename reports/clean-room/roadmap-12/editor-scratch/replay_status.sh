#!/bin/sh
# For each commit, rerun status_table.py on a git archive copy and count
# the status.md lines that change once the Similarity cell and the line
# that names the latest full similarity report are set aside.
# Usage: replay_status.sh SCRATCH_DIR COMMIT...
S="$1"; shift
cd /home/ryanm/Code/tidal || exit 1
for c in "$@"; do
  d="$S/st/$c"; mkdir -p "$d"; git archive "$c" | tar -x -C "$d"
  sed -e 's/| [^|]*([^|]*reports\/similarity[^|]*) |$/|/' -e '/latest similarity report that checked every page/d' "$d/documentation/status.md" > "$d/a"
  (cd "$d" && python3 -I -B tools/status_table.py >/dev/null 2>&1)
  sed -e 's/| [^|]*([^|]*reports\/similarity[^|]*) |$/|/' -e '/latest similarity report that checked every page/d' "$d/documentation/status.md" > "$d/b"
  echo "$c: $(diff "$d/a" "$d/b" | grep -c '^[<>]') lines differ outside the similarity cells"
  diff "$d/a" "$d/b" | grep '^[<>]' | cut -c1-200 | head -4
  rm -rf "$d"
done
