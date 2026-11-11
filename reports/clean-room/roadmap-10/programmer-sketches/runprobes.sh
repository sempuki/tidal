#!/bin/bash
# Runs the follow-up probes of the report session (results-probes.txt).
# runall.sh's OUT must point at a front end built from the commit read.
cd "$(dirname "$0")/probes"
for f in *.cpp; do echo "=== probes/$f"; ../runall.sh "$f"; done
