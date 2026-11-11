#!/bin/bash
# Reruns the round 10 programs, reductions, probes, short programs, bugs,
# and mistakes on the round 24 front end.
cd "$(dirname "$0")/round10"
for f in new/*.cpp reductions/*.cpp probes/*.cpp new/one-line/*.cpp new/bugs/*.cpp new/mistakes/*.cpp; do
  echo "=== round10/$f"; ../runall.sh "$f"
done
