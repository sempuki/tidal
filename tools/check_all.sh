#!/bin/sh
# Run every check before pushing: specification cross-references, the page
# examples on the baseline compilers, the full compiler matrix, the
# similarity check (when TIDAL_REFERENCE points at the reference corpus, see
# reports/README.md), the clean-room front ends' conformance against their
# baseline (when cargo is installed; each test that takeover.tsv lists
# takes the new front end's status, so a failing owned test fails here),
# and the HTML site build.
set -e
cd "$(dirname "$0")/.."
python3 -I tools/check_specification.py
python3 -I tools/check_examples.py
python3 -I tools/run_tests.py --report tests/RESULTS.md
if [ -n "$TIDAL_REFERENCE" ]; then
  python3 -I tools/similarity.py --fail-on-flag --reference "$TIDAL_REFERENCE" > build/similarity.txt
  tail -1 build/similarity.txt
else
  echo "similarity: skipped (set TIDAL_REFERENCE to the reference corpus)"
fi
python3 -I tools/frontend_check.py
python3 -I tools/build_site.py --strict
