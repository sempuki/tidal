#!/bin/sh
# Files that a commit changes and that the commits between its stated
# base and its parent also change. Usage: overlap.sh BASE COMMIT
cd /tmp/claude-1000/-home-ryanm-Code-tidal/34cd0ee5-eea0-4b25-b33b-c46b2846603b/scratchpad/fe28c || exit 1
git show --format= --name-only "$2" | sort > /tmp/claude-1000/-home-ryanm-Code-tidal/34cd0ee5-eea0-4b25-b33b-c46b2846603b/scratchpad/r15e/ov.a
git diff --name-only "$1" "$2^" | sort > /tmp/claude-1000/-home-ryanm-Code-tidal/34cd0ee5-eea0-4b25-b33b-c46b2846603b/scratchpad/r15e/ov.b
comm -12 /tmp/claude-1000/-home-ryanm-Code-tidal/34cd0ee5-eea0-4b25-b33b-c46b2846603b/scratchpad/r15e/ov.a /tmp/claude-1000/-home-ryanm-Code-tidal/34cd0ee5-eea0-4b25-b33b-c46b2846603b/scratchpad/r15e/ov.b
