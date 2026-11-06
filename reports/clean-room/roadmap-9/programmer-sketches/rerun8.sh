#!/bin/bash
cd "$(dirname "$0")/round8"
for f in *.cpp paren-pack/*.cpp one-line/*.cpp bugs/*.cpp mistakes/*.cpp; do
  echo "=== $f"; ../runall.sh "$f"
done
echo "=== logger/main.cpp logger/sink.cpp"; ../runall.sh logger/main.cpp logger/sink.cpp
