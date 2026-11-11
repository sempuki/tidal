#!/bin/bash
# Reruns the round 8 copies and the round 9 sketches on the round 23 front end.
cd "$(dirname "$0")/round8"
for f in *.cpp paren-pack/*.cpp one-line/*.cpp bugs/*.cpp mistakes/*.cpp; do
  echo "=== round8/$f"; ../runall.sh "$f"
done
echo "=== round8/logger/main.cpp logger/sink.cpp"; ../runall.sh logger/main.cpp logger/sink.cpp
cd ../round9
for f in new/*.cpp reductions/*.cpp new/one-line/*.cpp new/bugs/*.cpp new/mistakes/*.cpp; do
  echo "=== round9/$f"; ../runall.sh "$f"
done
