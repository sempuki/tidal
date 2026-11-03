#!/usr/bin/env python3
"""Rebuild the index table in questions/README.md from the question files.

Each row comes from one file questions/Q-NNNN-<slug>.md:
- the question is the text after "# Q-NNNN: " on the first line,
- the status is the "**Status:**" line,
- the answer is the first sentence of the "Provisional answer" section
  (or the "Answer" section of a settled question), without its final
  period and without a closing "(option N)".

The first sentence of the answer section states the answer in words, for
this column. The tool keeps the text above and below the table.

Fails when a file has no title, status, or answer section, when a status
is unknown, or when an answer starts with an option number ("Option 1.").

Usage:
    tools/question_index.py [--check]

--check writes nothing and fails when the table is out of date. It also
warns, without failing, about each question that a file in reports/review/
names when the question's Opinions section does not name that file. Each
commit that acts on a review records the review's reading on every
question the review read (decision 0054).
"""

import argparse
import re
import sys
from dataclasses import dataclass
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
QUESTIONS = ROOT / "questions"
README = QUESTIONS / "README.md"
REVIEWS = ROOT / "reports" / "review"
QUESTION_ID = re.compile(r"\bQ-\d{4}\b")
HEADER = "| ID | Question | Status | Provisional answer |"
STATUSES = ["open", "provisional", "compiler defect", "settled"]
TITLE = re.compile(r"^# (Q-\d{4}): (.+?)\s*$")
STATUS = re.compile(r"^\*\*Status:\*\*\s*(.+?)\s*$", re.M)
ANSWER = re.compile(r"^## (?:Provisional answer|Answer)[ \t]*\n(.*?)(?=^## |\Z)", re.M | re.S)
# A sentence ends at . ? or ! followed by a space and a capital letter
# or a backtick.
SENTENCE_END = re.compile(r"(?<=[.?!])\s+(?=[A-Z`])")
OPTION_TAIL = re.compile(r"\s*\(option \d+\)$")
ONLY_OPTION = re.compile(r"(?i)option \d+\b")


@dataclass
class Question:
    id: str
    path: Path
    title: str
    status: str
    answer: str


def first_sentence(section: str) -> str:
    paragraph = " ".join(section.strip().split("\n\n")[0].split())
    sentence = SENTENCE_END.split(paragraph, maxsplit=1)[0]
    sentence = sentence.rstrip(".")
    return OPTION_TAIL.sub("", sentence)


def load() -> tuple[list[Question], list[str]]:
    """Return the numbered questions in order, and the problems found."""
    questions, problems = [], []
    for path in sorted(QUESTIONS.glob("Q-[0-9][0-9][0-9][0-9]-*.md")):
        text = path.read_text()
        where = path.relative_to(ROOT)
        title = TITLE.match(text.split("\n", 1)[0])
        status = STATUS.search(text)
        answer = ANSWER.search(text)
        if not title or title.group(1) != path.name[:6]:
            problems.append(f"{where}: the first line must be '# {path.name[:6]}: <question>'")
            continue
        if not status:
            problems.append(f"{where}: no **Status:** line")
            continue
        if status.group(1) not in STATUSES:
            problems.append(f"{where}: unknown status {status.group(1)!r}")
        if not answer or not answer.group(1).strip():
            problems.append(f"{where}: no 'Provisional answer' section")
            continue
        sentence = first_sentence(answer.group(1))
        if ONLY_OPTION.match(sentence):
            problems.append(f"{where}: the answer's first sentence starts with an option number; "
                            "state the answer in words")
        questions.append(Question(title.group(1), path, title.group(2), status.group(1), sentence))
    for path in sorted(QUESTIONS.glob("Q-NEW-*.md")):
        print(f"warning: {path.relative_to(ROOT)} has a placeholder ID; "
              "run tools/number_questions.py", file=sys.stderr)
    return questions, problems


def review_warnings(questions: list[Question]) -> list[str]:
    """Each question that a review names and whose Opinions section does not name the review."""
    by_id = {q.id: q for q in questions}
    warnings = []
    for review in sorted(REVIEWS.glob("*.md")):
        for qid in sorted(set(QUESTION_ID.findall(review.read_text()))):
            if qid not in by_id:
                continue
            text = by_id[qid].path.read_text()
            opinions = text.split("\n## Opinions", 1)[1] if "\n## Opinions" in text else ""
            if review.name not in opinions:
                warnings.append(f"{by_id[qid].path.relative_to(ROOT)}: {review.relative_to(ROOT)} "
                                "names the question, and no Opinions entry names the review")
    return warnings


def cell(text: str) -> str:
    return text.replace("|", r"\|")


def render(text: str, questions: list[Question]) -> str:
    """Return the README text with the index table rebuilt."""
    lines = text.split("\n")
    start = lines.index(HEADER) + 2
    end = next((i for i in range(start, len(lines)) if not lines[i].startswith("|")), len(lines))
    lines[start:end] = [f"| [{q.id}]({q.path.name}) | {cell(q.title)} | {q.status} | {cell(q.answer)} |"
                        for q in questions]
    return "\n".join(lines)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--check", action="store_true", help="fail if the table is out of date")
    args = ap.parse_args()
    questions, problems = load()
    old = README.read_text()
    new = render(old, questions)
    if args.check:
        if new != old:
            problems.append("questions/README.md: the index is out of date; run tools/question_index.py")
        for w in review_warnings(questions):
            print(f"warning: {w}", file=sys.stderr)
    elif new != old:
        README.write_text(new)
        print("questions/README.md: index rebuilt")
    for p in problems:
        print(p)
    print(f"{len(questions)} questions, {len(problems)} problems")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
