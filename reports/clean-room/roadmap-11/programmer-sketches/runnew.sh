#!/bin/bash
# Runs the round 11 programs, reductions, probes, short programs, bugs, and mistakes.
cd "$(dirname "$0")"
for f in new/*.cpp reductions/*.cpp probes/*.cpp new/one-line/*.cpp new/bugs/*.cpp new/mistakes/*.cpp; do
  echo "=== $f"; ./runall.sh "$f"
done
