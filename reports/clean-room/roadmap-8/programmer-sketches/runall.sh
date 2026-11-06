#!/bin/bash
# Usage: runall.sh SOURCE... (several sources form one program)
# Builds with g++ and clang++ (plain, then sanitizers), runs each binary,
# checks with EDG, and runs tidal-fe on the first source.
OUT=/tmp/claude-1000/-home-ryanm-Code-tidal/558bea91-9fbf-4b01-a330-0a1bb4167438/scratchpad/r8-programmer
FE=$OUT/target/release/tidal-fe
B=$(mktemp -d $OUT/bin.XXXX)
FLAGS="-std=c++23 -pedantic-errors -Wall -Wextra"
SRCS=("$@")
for cxx in g++ clang++; do
  for san in "" "-fsanitize=undefined,address -fno-sanitize-recover=all"; do
    tag="$cxx${san:+ san}"
    if $cxx $FLAGS $san "${SRCS[@]}" -o $B/a 2>$B/err; then
      w=$(grep -c 'warning' $B/err)
      $B/a >/dev/null 2>$B/rerr; rc=$?
      echo "$tag: built (warnings $w), exit $rc $(head -c 200 $B/rerr | tr '\n' ' ')"
    else
      echo "$tag: FAILED: $(grep -m2 -E 'error' $B/err | tr '\n' ' ' | cut -c1-300)"
    fi
  done
done
edg=""
for s in "${SRCS[@]}"; do
  d=$(dirname "$(realpath "$s")")
  r=$(podman run --rm -v "$d":/w:z localhost/tidal-cxx-edg-7.0 cpfe --c++23 --strict --no_code_gen --instantiate=used /w/$(basename "$s") 2>&1)
  if [ $? -eq 0 ]; then edg="$edg accepts"; else edg="$edg rejects: $(echo "$r" | grep -m1 error | cut -c1-200)"; fi
done
echo "EDG:$edg"
if [ ${#SRCS[@]} -gt 1 ]; then
  echo "tidal-fe: $($FE run "${SRCS[@]}" 2>&1 | head -5 | tr '\n' ' ' | cut -c1-400)"
else
  echo "tidal-fe: $($FE run "${SRCS[0]}" 2>&1 | head -5 | tr '\n' ' ' | cut -c1-400)"
fi
rm -rf $B
