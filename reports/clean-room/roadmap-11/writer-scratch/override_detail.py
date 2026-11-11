"""Print the tests and override lines of the given rules. Read-only."""
import sys
sys.path.insert(0, "tools")
import check_specification as cs
rules, _ = cs.parse_spec()
for rid in sys.argv[1:]:
    f, cls, tests = rules[rid]
    print(f"== {rid} ({cls}) {f.relative_to(cs.ROOT)}")
    for t in tests:
        hdr = [l for l in (cs.TESTS / t).read_text().splitlines() if l.startswith("// tidal-expect") or l.startswith("// tidal-note")]
        print("  ", t)
        for l in hdr:
            print("     ", l[:220])
