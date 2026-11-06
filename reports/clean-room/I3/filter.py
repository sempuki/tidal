#!/usr/bin/env python3
"""Print a spec page with every '## Evidence' segment and every :tests: line removed."""
import re, sys
for path in sys.argv[1:]:
    skip = False
    out = []
    for line in open(path, encoding="utf-8"):
        m = re.match(r'^(#{1,6})\s', line)
        if m:
            skip = line.strip().lower().startswith('## evidence') or (skip and len(m.group(1)) > 2)
            if line.strip().lower().startswith('## evidence'):
                skip = True
        if skip:
            continue
        if ':tests:' in line:
            continue
        out.append(line)
    text = ''.join(out)
    assert ':tests:' not in text and '## Evidence' not in text
    sys.stdout.write(f"===== {path}\n" + text)
