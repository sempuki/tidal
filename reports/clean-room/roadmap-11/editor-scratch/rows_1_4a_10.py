"""Distinct open questions in the packet rows 1, 4a, and 10 ("answers stay").
Usage: rows_1_4a_10.py PENDING_MD"""
import re, sys
rows = {}
for l in open(sys.argv[1]).read().splitlines():
    m = re.match(r'^\| (1|4a|10) \| ([^|]*)\| ([^|]*)\|', l)
    if m and 'answers stay' in m.group(2) and 'rule items' not in m.group(2):
        rows[m.group(1)] = set(re.findall(r'Q-\d{4}', m.group(3)))
print({k: len(v) for k, v in rows.items()}, len(set().union(*rows.values())))
