#!/bin/sh
# Tests added and changed by each phase 13 commit (read-only).
cd /home/ryanm/Code/tidal || exit 1
for c in $(git rev-list --reverse c67b349b..3bd790ae); do
  a=$(git diff --name-status $c~1 $c -- tests | grep -c '^A.*\.cpp$')
  m=$(git diff --name-status $c~1 $c -- tests | grep -c '^M.*\.cpp$')
  echo "$(git log -1 --format=%h $c) added=$a modified=$m"
done
