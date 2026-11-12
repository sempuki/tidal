#!/bin/bash
# Reruns the round 14 programs, earlier tries, short programs, bugs,
# mistakes, reductions, and probes on both front ends, four files at a
# time, and writes results-rerun.txt in file order.
here=$(cd "$(dirname "$0")" && pwd)
cd "$here/round14"
tmp=$(mktemp -d)
ls new/*.cpp new/one-line/*.cpp new/bugs/*.cpp new/mistakes/*.cpp reductions/*.cpp probes/*.cpp > $tmp/list
cat -n $tmp/list | xargs -P 4 -n 2 sh -c '{ echo "=== round14/$1"; "'"$here"'/runall.sh" "$1"; } > '"$tmp"'/out.$0 2>/dev/null'
for i in $(seq 1 $(wc -l < $tmp/list)); do cat $tmp/out.$i; done > "$here/results-rerun.txt"
rm -rf $tmp
