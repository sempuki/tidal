#!/bin/sh
# Run every check before pushing: specification cross-references, the full compiler
# matrix, the similarity check (when TIDAL_REFERENCE points at the reference
# corpus, see reports/README.md), the clean-room front end's conformance
# against its baseline (when cargo is installed), and the HTML site build.
set -e
cd "$(dirname "$0")/.."
python3 -I tools/check_specification.py
python3 -I tools/run_tests.py --report tests/RESULTS.md
if [ -n "$TIDAL_REFERENCE" ]; then
  python3 -I tools/similarity.py --reference "$TIDAL_REFERENCE" | tail -1
else
  echo "similarity: skipped (set TIDAL_REFERENCE to the reference corpus)"
fi
python3 -I tools/frontend_check.py
python3 -I tools/build_site.py --strict
