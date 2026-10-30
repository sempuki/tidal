#!/bin/sh
# Run every check before pushing: spec cross-references, the full compiler
# matrix, the similarity check (when TIDAL_REFERENCE points at the reference
# corpus, see reports/README.md), and the PDF build.
set -e
cd "$(dirname "$0")/.."
python3 -I tools/check_spec.py
python3 -I tools/run_tests.py --report tests/RESULTS.md
if [ -n "$TIDAL_REFERENCE" ]; then
  python3 -I tools/similarity.py --reference "$TIDAL_REFERENCE" | tail -1
else
  echo "similarity: skipped (set TIDAL_REFERENCE to the reference corpus)"
fi
python3 -I tools/build_pdf.py
