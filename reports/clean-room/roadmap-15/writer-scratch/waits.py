"""Tally the **Waits for:** items of open questions. Read-only."""
import re, collections, pathlib
items = collections.Counter(); whole = collections.Counter(); rows = collections.Counter()
for f in sorted(pathlib.Path("questions").glob("Q-*.md")):
    t = f.read_text()
    if not re.search(r"^\*\*Status:\*\* provisional", t, re.M):
        continue
    m = re.search(r"^\*\*Waits for:\*\* (.*)$", t, re.M)
    w = m.group(1) if m else "(none)"
    parts = [p.strip() for p in w.split(";")]
    for p in parts:
        k = re.sub(r"\(pending\.md row (\w+)\)", "", p).strip() if "maintainer" in p else p
        items[k] += 1
        if r := re.search(r"row (\w+)", p):
            rows[r.group(1)] += 1
    whole[" + ".join(sorted(re.sub(r" \(pending.*", "", p) for p in parts))] += 1
for k, v in items.most_common(): print(f"{v:4} {k}")
print("--- whole field")
for k, v in whole.most_common(): print(f"{v:4} {k}")
print("--- rows", dict(rows))
