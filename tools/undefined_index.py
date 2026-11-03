#!/usr/bin/env python3
"""The table of `undefined` rules and the compilers whose constant evaluation rejects each one.

The site's `tidal-undefined-index` directive (site/_ext/tidal.py) renders
the table as HTML. tools/exercises.py renders it as Markdown in the copy
of the specification that a clean-room reader gets, because the copy has
no directive processing. Both use rows() so that the two agree.

A compiler rejects a rule's case if a test of the rule expects `reject`
from it. Tests with -Werror check a warning, not constant evaluation,
so they don't count.
"""

import fnmatch
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import matrix  # noqa: E402
import run_tests  # noqa: E402

FAMILIES = (("clang", "Clang"), ("gcc", "GCC"), ("edg", "EDG"))
RULE_LINE = re.compile(r"^:::\{rule\}\s+([A-Z]+-\d+)\s*$")
DIRECTIVE = ":::{tidal-undefined-index}\n:::\n"


def undefined_rules(root: Path):
    """Yield (rule id, page path, first sentence) for each `undefined` rule, in page order."""
    for page in sorted(root.rglob("*.md")):
        lines = page.read_text().splitlines()
        for i, line in enumerate(lines):
            m = RULE_LINE.match(line)
            if not m:
                continue
            j = i + 1
            kind = None
            while j < len(lines) and lines[j].startswith(":"):
                if lines[j].startswith(":kind:"):
                    kind = lines[j].split(":", 2)[2].strip()
                j += 1
            if kind != "undefined":
                continue
            text = []
            for l in lines[j:]:
                if l == ":::" or (text and not l.strip()):
                    break
                if l.strip():
                    text.append(l.strip())
            first = re.split(r"(?<=[.:])\s", " ".join(text))[0]
            yield m.group(1), page, first


def mark(tests, compilers) -> str:
    """How the compilers of one family treat a rule's rejection tests.

    Each compiler rejects every case, some cases, or none. A family whose
    compilers differ lists them by id.
    """
    cases = [t for t in tests if t.default == "reject" or "reject" in t.overrides.values()]
    if not cases:
        return "no test"

    def status(c):
        ran = [t for t in cases
               if not any(fnmatch.fnmatch(f, p) for f in t.flags for p in c.unsupported_flags)]
        n = sum(t.expect(c) == "reject" for t in ran)
        return "rejects" if n == len(ran) else "accepts" if n == 0 else "some cases"

    by_status = {}
    for c in compilers:
        by_status.setdefault(status(c), []).append(c.id)
    if len(by_status) == 1:
        return next(iter(by_status))
    return "; ".join(f"{', '.join(ids)}: {st}" for st, ids in by_status.items())


def test_paths() -> list[Path]:
    """Every test file, which the table reads."""
    return run_tests.test_files([matrix.TESTS])


def rows(root: Path):
    """Yield (rule id, page path, first sentence, {family: mark}) for each `undefined` rule."""
    compilers = [c for c in matrix.load() if c.role == "baseline"]
    tests = {}
    for path in test_paths():
        t = run_tests.parse_test(path)
        if "-Werror" in t.flags:
            continue
        for rid in t.rules:
            tests.setdefault(rid, []).append(t)
    for rid, page, first in undefined_rules(root):
        marks = {family: mark(tests.get(rid, []), [c for c in compilers if c.family == family])
                 for family, _ in FAMILIES}
        yield rid, page, first, marks


def markdown(root: Path, here: Path) -> str:
    """The table as Markdown, with links relative to the page `here`."""
    import os
    lines = ["| Rule | Operation | " + " | ".join(name for _, name in FAMILIES) + " |",
             "|---|---|" + "---|" * len(FAMILIES)]
    for rid, page, first, marks in rows(root):
        link = os.path.relpath(page, here.parent)
        cells = [f"[{rid}]({link})", first.replace("|", "\\|")]
        cells += [marks[family] for family, _ in FAMILIES]
        lines.append("| " + " | ".join(cells) + " |")
    return "\n".join(lines) + "\n"
