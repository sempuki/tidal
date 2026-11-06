#!/bin/bash
cd "$(dirname "$0")"
for f in new/*.cpp reductions/*.cpp new/one-line/*.cpp new/bugs/*.cpp new/mistakes/*.cpp; do
  echo "=== $f"; ./runall.sh "$f"
done
