"""The agreement sentence in each page's Evidence segment.

Each page says how many of its tests the baseline compilers agree on:
"All eight compilers agree on 63 of 69 tests." A test counts as agreed
when it carries no per-compiler expectation (tidal-expect-<x>) other
than one for a report compiler. A compiler that skips a test, as EDG
skips -Werror tests, does not disagree. tools/status_table.py rewrites
the sentences, and tools/check_specification.py fails when one is out of date.
"""

import re
import textwrap
from pathlib import Path

import matrix

NUMBERS = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight",
           "nine", "ten", "eleven", "twelve"]
SENTENCE = re.compile(
    r"All\s+\w+\s+compilers(?:\s+in\s+the\s+matrix)?\s+agree\s+on\s+"
    r"(?:all\s+\d+\s+tests|\d+\s+of\s+\d+\s+tests|every\s+test,\s+except\s+where\s+a\s+"
    r"question\s+is\s+noted)\.")


def counts(page_text: str) -> tuple[int, int]:
    """Return (tests with no per-compiler expectation, all tests) for a page."""
    tests = set()
    for line in re.findall(r"^:tests: (.*)$", page_text, flags=re.M):
        tests.update(line.split())
    report = {c.id for c in matrix.load() if c.role == "report"}

    def agrees(text: str) -> bool:
        keys = re.findall(r"^// tidal-expect-([a-z0-9.-]+):", text, flags=re.M)
        return all(k in report for k in keys)

    agreed = sum(1 for t in tests if (matrix.TESTS / t).exists()
                 and agrees((matrix.TESTS / t).read_text()))
    return agreed, len(tests)


def sentence(page_text: str) -> str:
    agreed, total = counts(page_text)
    n = sum(1 for c in matrix.load() if c.role == "baseline")
    word = NUMBERS[n] if n < len(NUMBERS) else str(n)
    if agreed == total:
        return f"All {word} compilers agree on all {total} tests."
    return f"All {word} compilers agree on {agreed} of {total} tests."


def current(page_text: str) -> bool:
    """True when the page has no agreement sentence or its sentence is up to date."""
    m = SENTENCE.search(page_text)
    return m is None or " ".join(m.group(0).split()) == sentence(page_text)


def rewrite(page: Path) -> bool:
    """Rewrite the page's agreement sentence and rewrap its paragraph. True if changed."""
    text = page.read_text()
    m = SENTENCE.search(text)
    if m is None or current(text):
        return False
    start = text.rfind("\n\n", 0, m.start()) + 2
    end = text.find("\n\n", m.end())
    end = len(text) if end < 0 else end
    para = text[start:m.start()] + sentence(text) + text[m.end():end]
    protected = re.sub(r"`[^`]*`", lambda c: c.group(0).replace(" ", "\0"), " ".join(para.split()))
    wrapped = "\n".join(textwrap.wrap(protected, 72, break_long_words=False,
                                      break_on_hyphens=False)).replace("\0", " ")
    page.write_text(text[:start] + wrapped + text[end:])
    return True
