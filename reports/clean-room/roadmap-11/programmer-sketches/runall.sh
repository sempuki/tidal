#!/bin/bash
# Usage: runall.sh SOURCE... (several sources form one program)
# Builds with g++ and clang++ (the Write flags, -pedantic-errors alone,
# then sanitizers), runs each binary, checks with EDG, and runs tidal-fe.
OUT=/tmp/claude-1000/-home-ryanm-Code-tidal/34cd0ee5-eea0-4b25-b33b-c46b2846603b/scratchpad/r11
FE=$OUT/release/tidal-fe
B=$(mktemp -d $OUT/bin.XXXX)
SRCS=("$@")
for cxx in g++ clang++; do
  for mode in wall ped san; do
    case $mode in
      wall) fl="-std=c++23 -pedantic-errors -Wall -Wextra"; tag="$cxx";;
      ped) fl="-std=c++23 -pedantic-errors"; tag="$cxx ped";;
      san) fl="-std=c++23 -pedantic-errors -Wall -Wextra -fsanitize=undefined,address -fno-sanitize-recover=all"; tag="$cxx san";;
    esac
    if $cxx $fl "${SRCS[@]}" -o $B/a 2>$B/err; then
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
echo "tidal-fe: $(timeout 120 $FE run "${SRCS[@]}" 2>&1 | head -5 | tr '\n' ' ' | cut -c1-400)"
rm -rf $B
