#!/bin/bash
# Runs the new programs, short programs, bugs, mistakes, reductions, and
# probes of this round on every tool, four files at a time, and writes
# results-new.txt in file order.
here=$(cd "$(dirname "$0")" && pwd)
cd "$here"
tmp=$(mktemp -d)
ls new/*.cpp new/one-line/*.cpp new/bugs/*.cpp new/mistakes/*.cpp reductions/*.cpp probes/*.cpp 2>/dev/null > $tmp/list
cat -n $tmp/list | xargs -P 4 -n 2 sh -c '{ echo "=== $1"; "'"$here"'/runall.sh" "$1"; } > '"$tmp"'/out.$0 2>/dev/null'
for i in $(seq 1 $(wc -l < $tmp/list)); do cat $tmp/out.$i; done > "$here/results-new.txt"
rm -rf $tmp
