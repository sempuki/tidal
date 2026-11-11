#!/usr/bin/env python3
"""Print a specification page with every '## Evidence' segment and every :tests: line removed.
An Evidence segment runs from its heading to the next heading of the same or a higher level."""
import re, sys
for path in sys.argv[1:]:
    print(f"===== {path}")
    skip_level = None
    for line in open(path, encoding="utf-8"):
        m = re.match(r"^(#{1,6})\s+(.*)", line)
        if m:
            level = len(m.group(1))
            if skip_level is not None and level <= skip_level:
                skip_level = None
            if skip_level is None and re.match(r"Evidence\b", m.group(2)):
                skip_level = level
        if skip_level is not None:
            continue
        if ":tests:" in line:
            continue
        sys.stdout.write(line)
