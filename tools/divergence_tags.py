"""The status tag of each line in a page's Divergences part.

A Divergences line starts `- {question}`Q-NNNN` (RULES; TAG):`. The tag
restates the question's status: `open` for a provisional question,
`compiler defect`, or `settled` (with any words after it, such as
"settled on ENUM-P3", kept). tools/status_table.py rewrites a tag that
disagrees with the question file, and tools/check_specification.py
fails on one (decision 0058).
"""

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
QUESTIONS = ROOT / "questions"
LINE = re.compile(r"(- \{question\}`(Q-\d{4})` \([^;()]*;\s+)([^)]*?)(\))")
STATUS = re.compile(r"^\*\*Status:\*\*\s*(.+?)\s*$", re.M)


def statuses() -> dict[str, str]:
    out = {}
    for f in QUESTIONS.glob("Q-[0-9][0-9][0-9][0-9]-*.md"):
        if m := STATUS.search(f.read_text()):
            out[f.name[:6]] = m.group(1)
    return out


def tag_for(status: str, old: str) -> str:
    words = " ".join(old.split())
    if status == "provisional":
        return "open"
    if status == "compiler defect":
        return "compiler defect"
    if status == "settled":
        return words if words.startswith("settled") else "settled"
    return words


def render(text: str, known: dict[str, str]) -> str:
    """The page with each Divergences tag set from its question's status."""
    if "### Divergences" not in text:
        return text
    head, rest = text.split("### Divergences", 1)
    part, sep, tail = rest.partition("\n### ")

    def fix(m: re.Match) -> str:
        status = known.get(m.group(2))
        if status is None:
            return m.group(0)
        new = tag_for(status, m.group(3))
        if new == " ".join(m.group(3).split()):
            return m.group(0)
        return m.group(1) + new + m.group(4)

    return head + "### Divergences" + LINE.sub(fix, part) + sep + tail


def current(page: Path, known: dict[str, str]) -> bool:
    text = page.read_text()
    return render(text, known) == text


def rewrite(page: Path, known: dict[str, str]) -> bool:
    text = page.read_text()
    new = render(text, known)
    if new == text:
        return False
    page.write_text(new)
    return True
