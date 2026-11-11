"""List the open questions whose **Waits for:** line names only a review,
with the kind, the first rule of the Affects line, and the date of the
last dated note. Read-only. Run from the repository root."""
import re, pathlib, collections
by_kind = collections.Counter(); by_prefix = collections.Counter()
for f in sorted(pathlib.Path("questions").glob("Q-*.md")):
    t = f.read_text()
    if not re.search(r"^\*\*Status:\*\* provisional", t, re.M):
        continue
    w = re.search(r"^\*\*Waits for:\*\* (.*)$", t, re.M).group(1).strip()
    m = re.fullmatch(r"a review \((\w+)\)", w)
    if not m:
        continue
    aff = re.search(r"^\*\*Affects:\*\* (.*)$", t, re.M).group(1)
    rule = (re.findall(r"\b([A-Z]+)-P?\d+", aff) or ["?"])[0]
    by_kind[m.group(1)] += 1; by_prefix[(m.group(1), rule)] += 1
    print(f.name[:6], m.group(1), rule)
print(dict(by_kind))
for k, v in sorted(by_prefix.items()): print(k, v)
