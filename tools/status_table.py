#!/usr/bin/env python3
"""Regenerate the specification table in docs/status.md from the pages.

The table lists every page that a section index includes, in site order,
with its rule count, rule classes, test count, and questions. It replaces
the lines between the table header and the first blank line after it.

Usage:
    tools/status_table.py
"""

import re
import sys
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SPEC = ROOT / "spec"
TESTS = ROOT / "tests"
STATUS = ROOT / "docs" / "status.md"
KINDS = ["required", "reject", "undefined", "unspecified", "impl-defined",
         "no-diagnostic", "recommended"]
HEADER = "| Section | Rules | Classes | Tests | Questions | Similarity |"


def pages(index: Path):
    """Yield page paths in the order the index lists them, recursing."""
    text = index.read_text()
    m = re.search(r":::\{tidal-pages\}\n(.*?)\n:::", text, re.S)
    if not m:
        return
    for name in m.group(1).split():
        target = index.parent / f"{name}.md"
        if name.endswith("index"):
            yield from pages(index.parent / f"{name}.md")
        elif target.exists():
            yield target


def row(page: Path) -> str:
    src = page.read_text()
    title = src.splitlines()[0][2:].strip()
    kinds = Counter(re.findall(r"^:kind: (\S+)", src, flags=re.M))
    tests = set()
    for line in re.findall(r"^:tests: (.*)$", src, flags=re.M):
        tests.update(line.split())
    classes = ", ".join(f"{kinds[k]} {k}" for k in KINDS if kinds[k])
    questions = ", ".join(sorted(set(re.findall(r"Q-\d{4}", src))))
    rel = page.relative_to(ROOT)
    return f"| [{title}](../{rel}) | {sum(kinds.values())} | {classes} | {len(tests)} | {questions} | 0 flagged |"


def main() -> int:
    rows = []
    for section in ["statements", "expressions", "declarations", "classes"]:
        rows += [row(p) for p in pages(SPEC / section / "index.md")]
    glossary = (SPEC / "glossary.md").read_text()
    terms = len(re.findall(r"^\S.*$", glossary.split(":::{glossary}")[1].split(":::")[0], flags=re.M))
    rows.append(f"| [Glossary](../spec/glossary.md) | {terms} terms | | | | 0 flagged |")
    lines = STATUS.read_text().split("\n")
    start = lines.index(HEADER) + 2
    end = next(i for i in range(start, len(lines)) if not lines[i].startswith("|"))
    lines[start:end] = rows
    STATUS.write_text("\n".join(lines))
    print(f"{len(rows)} rows")
    return 0


if __name__ == "__main__":
    sys.exit(main())
