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

Each provisional question has one "**Waits for:**" line in its header,
and no other question has one (decision 0057). The line names what holds
the question open, as items joined by "; ". Each item is one of the
WAITS_FOR forms below. An item that names a row of pending.md fails
unless that row, or the section the row points to, names the question.

Usage:
    tools/question_index.py [--check]

The tool also writes the number of questions settled under decision 0054
into row 1 of documentation/decisions/pending.md ("Confirms the N
questions settled under decision 0054"). A question counts when its file
says "Settled on DATE under decision 0054".

The tool also writes the "Decision packet" section of pending.md from
the **Waits for:** lines: for each row, the open questions that wait on
it, and the tests whose tidal-note lines name those questions. Row 1
holds the questions that wait for "newest families that agree".

--check writes nothing. It fails when the table is out of date, when the
count in pending.md is wrong, and when a file in reports/review/ names a
question whose Opinions section does not name that file. Each commit
that acts on a review records the review's reading on every question the
review read (decision 0054).
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
PENDING = ROOT / "documentation" / "decisions" / "pending.md"
SETTLED_0054 = re.compile(r"Settled on \d{4}-\d{2}-\d{2} under decision 0054")
PENDING_COUNT = re.compile(r"(Confirms the )(\d+)( questions settled under decision 0054)")
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
WAITS_LINE = re.compile(r"^\*\*Waits for:\*\*\s*(.*?)\s*$", re.M)
PRINCIPLE_IDS = r"[A-Z]+-P\d+(?: [A-Z]+-P\d+)*"
WAITS_FOR = [
    re.compile(r"the maintainer \(pending\.md row (\d+(?:, row \d+)*)\)"),
    re.compile(rf"a use of {PRINCIPLE_IDS}"),
    re.compile(r"newest families that agree"),
    re.compile(r"a review \((?:doubt|silent|against|part)\)"),
    re.compile(r"a first review"),
    re.compile(r"a principle \([A-Z]+-P\d+\)"),
    re.compile(r"a rule"),
    re.compile(r"child questions \(Q-\d{4}(?:, Q-\d{4})*\)"),
]
# The sections of pending.md that a row points to.
PENDING_SECTIONS = {"Matrix against review": 4, "EDG report": 3,
                    "Later-version probe": 7, "Process questions": 9}


@dataclass
class Question:
    id: str
    path: Path
    title: str
    status: str
    answer: str
    waits_for: list[str]


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
        waits = WAITS_LINE.findall(text.split("\n## ", 1)[0])
        if status.group(1) == "provisional" and len(waits) != 1:
            problems.append(f"{where}: a provisional question needs one **Waits for:** line in its header")
        elif status.group(1) != "provisional" and waits:
            problems.append(f"{where}: only a provisional question has a **Waits for:** line")
        items = waits[0].split("; ") if waits else []
        for item in items:
            if not any(form.fullmatch(item) for form in WAITS_FOR):
                problems.append(f"{where}: unknown **Waits for:** item {item!r}")
        questions.append(Question(title.group(1), path, title.group(2), status.group(1), sentence, items))
    for path in sorted(QUESTIONS.glob("Q-NEW-*.md")):
        print(f"warning: {path.relative_to(ROOT)} has a placeholder ID; "
              "run tools/number_questions.py", file=sys.stderr)
    return questions, problems


def settled_0054(questions: list[Question]) -> int:
    return sum(1 for q in questions if SETTLED_0054.search(q.path.read_text()))


def render_pending(text: str, count: int) -> str | None:
    """Return pending.md with the count of row 1 rewritten, or None when the phrase is missing."""
    if not PENDING_COUNT.search(text):
        return None
    return PENDING_COUNT.sub(lambda m: f"{m.group(1)}{count}{m.group(3)}", text, count=1)


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


def pending_rows(text: str) -> dict[str, set[int]]:
    """For each question ID, the rows of pending.md that name it, directly or by section."""
    rows: dict[str, set[int]] = {}
    section = None
    for line in text.splitlines():
        if line.startswith("## "):
            section = line[3:].strip()
        m = re.match(r"\| (\d+) \|", line)
        row = int(m.group(1)) if m and section is None else PENDING_SECTIONS.get(section)
        if row is not None:
            for qid in QUESTION_ID.findall(line):
                rows.setdefault(qid, set()).add(row)
    return rows


def waits_problems(questions: list[Question], pending: str) -> list[str]:
    """Each **Waits for:** row that pending.md does not hold the question in."""
    rows, problems = pending_rows(pending), []
    for q in questions:
        for item in q.waits_for:
            if m := WAITS_FOR[0].fullmatch(item):
                for row in re.findall(r"\d+", m.group(1)):
                    if int(row) not in rows.get(q.id, set()):
                        problems.append(f"{q.path.relative_to(ROOT)}: **Waits for:** names "
                                        f"pending.md row {row}, which does not name the question")
    return problems


PACKET_HEADING = "## Decision packet"
TESTS = ROOT / "tests"


def note_citations() -> dict[str, set[str]]:
    """For each question ID, the tests whose tidal-note lines name it."""
    cited: dict[str, set[str]] = {}
    for f in TESTS.rglob("*.cpp"):
        for line in f.read_text().splitlines():
            if not line.startswith("//"):
                break
            if line.startswith("// tidal-note:"):
                for q in QUESTION_ID.findall(line):
                    cited.setdefault(q, set()).add(str(f.relative_to(TESTS)))
    return cited


def row4_split(text: str) -> tuple[set[str], set[str]]:
    """Row 4's questions: those where every newest family keeps the matrix's answer (4a), and the rest (4b)."""
    section = text.split("## Matrix against review", 1)[1].split("\n## ", 1)[0]
    review: set[str] = set()
    named: set[str] = set()
    for line in section.splitlines():
        if not line.startswith("| ") or line.startswith("| Item"):
            continue
        cells = [c.strip() for c in line.strip("|").split("|")]
        qs = set(QUESTION_ID.findall(cells[0]))
        named |= qs
        if any(c.startswith("review") for c in cells[2:5]):
            review |= qs
    return named - review, review


def share_line(questions: list[Question], held: dict[int, list[str]], silent: list[str]) -> str:
    """The share of open questions that the rows of pending.md hold (decision 0059)."""
    open_ids = {q.id for q in questions if q.status == "provisional"}
    nine = {q for qs in held.values() for q in qs} & open_ids
    ten = (nine | set(silent)) & open_ids
    pct = lambda n: round(100 * n / len(open_ids)) if open_ids else 0
    return (f"Rows 1 to 9 hold {len(nine)} of the {len(open_ids)} open questions ({pct(len(nine))}%). "
            f"With row 10, the rows hold {len(ten)} ({pct(len(ten))}%).")


def render_packet(text: str, questions: list[Question]) -> str | None:
    """Return pending.md with the decision packet rewritten, or None when the heading is missing."""
    if PACKET_HEADING not in text:
        return None
    held: dict[int, list[str]] = {}
    silent: list[str] = []
    for q in questions:
        for item in q.waits_for:
            if m := WAITS_FOR[0].fullmatch(item):
                for row in re.findall(r"\d+", m.group(1)):
                    held.setdefault(int(row), []).append(q.id)
            elif item == "newest families that agree":
                held.setdefault(1, []).append(q.id)
            elif item == "a review (silent)":
                silent.append(q.id)
    cited = note_citations()

    def tests(qs: list[str]) -> int:
        return len(set().union(*(cited.get(q, set()) for q in qs))) if qs else 0

    def stay(qs: list[str]) -> str:
        return f"{tests(qs)} | 0"

    def change(qs: list[str]) -> str:
        n = tests(qs)
        return f"{n} | at most {n}" if n else "0 | 0"

    def listed(qs: list[str]) -> str:
        qs = sorted(set(qs))
        return f"{len(qs)}: {', '.join(qs)}" if qs else "none"

    four_a, four_b = row4_split(text)
    row4 = set(held.get(4, []))
    rows = sorted(set(int(m) for m in re.findall(r"^\| (\d+) \|", text.split("\n## ")[0], re.M)))
    lines = [PACKET_HEADING, "",
             "`tools/question_index.py` writes this section from the `**Waits for:**`",
             "line of each open question and from the row 4 table below. For each",
             "row and option, it gives the open questions that the option settles",
             "and the number of tests whose notes name them. \"Answers stay\" means",
             "that no test's expectation changes. For the other options, the last",
             "column gives an upper bound: the tests that name the questions, some of",
             "which hold a case that the option leaves as it is. The tool cannot tell",
             "which case each test holds. Row 4a holds the questions where every",
             "newest family keeps the matrix's answer, and row 4b those where one",
             "family follows the review. Row 10 holds the questions whose last",
             "review finds the reference silent.", "",
             "| Row | Option | Open questions it settles | Tests that name them | Tests whose expectation changes |",
             "|---|---|---|---|---|"]
    for row in rows:
        qs = held.get(row, [])
        if row == 1:
            lines.append(f"| 1 | Widen decision 0054 to settle while two newest families differ; answers stay | {listed(qs)} | {stay(qs)} |")
        elif row == 4:
            a = [q for q in row4 if q in four_a]
            b = [q for q in row4 if q not in four_a]
            lines.append(f"| 4a | Follow the compilers; answers stay | {listed(a)} | {stay(a)} |")
            lines.append(f"| 4a | Follow the review; answers change | {listed(a)} | {change(a)} |")
            lines.append(f"| 4b | Follow the compilers; answers stay | {listed(b)} | {stay(b)} |")
            lines.append(f"| 4b | Follow the review; answers change | {listed(b)} | {change(b)} |")
        elif row != 10:
            lines.append(f"| {row} | Decide the row; answers may change | {listed(qs)} | {change(qs)} |")
    lines.append(f"| 10 | Record each answer as Tidal's own choice; answers stay | {listed(silent)} | {stay(silent)} |")
    lines += ["", share_line(questions, held, silent), ""]
    kinds: dict[str, int] = {}
    for q in questions:
        for item in q.waits_for:
            if m := re.fullmatch(r"a review \((\w+)\)", item):
                kinds[m.group(1)] = kinds.get(m.group(1), 0) + 1
    if kinds:
        listed_kinds = ", ".join(f"{kinds[k]} {k}" for k in ("doubt", "silent", "against", "part") if k in kinds)
        lines += [f"{sum(kinds.values())} open questions wait for a review that supports the answer "
                  f"with no doubt. By what the last review found: {listed_kinds}.", ""]
    before, after = text.split(PACKET_HEADING, 1)
    rest = after[after.index("\n## "):] if "\n## " in after else ""
    return before + "\n".join(lines) + rest


NEWEST = ("clang-22", "gcc-16", "edg-7.0")


def rule_kinds() -> dict[str, str]:
    """The class of each rule, from the :kind: line under its rule directive."""
    kinds = {}
    for f in (ROOT / "specification").rglob("*.md"):
        for m in re.finditer(r"^:::\{rule\} (\S+)\n:kind: (\S+)", f.read_text(), re.M):
            kinds[m.group(1)] = m.group(2)
    return kinds


def family_warnings(questions: list[Question]) -> list[str]:
    """Questions whose "newest families that agree" item disagrees with their tests' overrides.

    For each open question, the tests whose notes name it give the most
    newest families that differ from the base expectation in one test.
    The item belongs on the line when that count is two or more.

    A test whose rules all have a class that allows either answer
    (no-diagnostic, unspecified, impl-defined, conditionally-supported)
    does not count, because no answer of a family breaks its rules. A
    question whose Affects line names a probe grid (tests/**/grids/*.json)
    gets no warning for a missing split, because the grid can hold the
    split and the tool does not run grids.
    """
    sys.path.insert(0, str(ROOT / "tools"))
    import matrix
    import run_tests
    comps = {c.id: c for c in matrix.load()}
    cited = note_citations()
    kinds = rule_kinds()
    either = {"no-diagnostic", "unspecified", "impl-defined", "conditionally-supported"}
    out = []
    for q in questions:
        if q.status != "provisional":
            continue
        most = 0
        for name in cited.get(q.id, ()):
            test = run_tests.parse_test(TESTS / name)
            if test.rules and all(kinds.get(r) in either for r in test.rules):
                continue
            base = test.default
            differ = 0
            for cid in NEWEST:
                c = comps[cid]
                expect = test.expect(c)
                own = "accept" if c.frontend_only and base in {"run", "run-fail", "link-fail"} else base
                differ += expect is not None and expect != own
            most = max(most, differ)
        has = "newest families that agree" in q.waits_for
        grid = "/grids/" in q.path.read_text().split("**Affects:**", 1)[-1].split("\n", 1)[0]
        if has and most < 2 and not grid:
            out.append(f"{q.path.relative_to(ROOT)}: names newest families that agree, and no test "
                       "that names it has two newest families that differ")
        elif not has and most >= 2:
            out.append(f"{q.path.relative_to(ROOT)}: a test that names it has {most} newest families "
                       "that differ, and its line does not name newest families that agree")
    return out


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
    ap.add_argument("--share", action="store_true",
                    help="print the share of open questions that pending.md holds, for a closing commit")
    args = ap.parse_args()
    questions, problems = load()
    old = README.read_text()
    new = render(old, questions)
    pending_old = PENDING.read_text()
    pending_new = render_pending(pending_old, settled_0054(questions))
    if pending_new is None:
        problems.append(f"{PENDING.relative_to(ROOT)}: row 1 has no phrase "
                        "'Confirms the N questions settled under decision 0054'")
        pending_new = pending_old
    packet = render_packet(pending_new, questions)
    if packet is None:
        problems.append(f"{PENDING.relative_to(ROOT)}: no '{PACKET_HEADING}' section")
    else:
        pending_new = packet
    problems.extend(waits_problems(questions, pending_old))
    if args.share:
        if m := re.search(r"^Rows 1 to 9 hold .*$", pending_new, re.M):
            print(m.group(0))
        return 0
    if args.check:
        if new != old:
            problems.append("questions/README.md: the index is out of date; run tools/question_index.py")
        if pending_new != pending_old:
            problems.append(f"{PENDING.relative_to(ROOT)}: the count of questions settled under "
                            "decision 0054 or the decision packet is out of date; run tools/question_index.py")
        problems.extend(review_warnings(questions))
        for w in family_warnings(questions):
            print(f"warning: {w}")
    else:
        if new != old:
            README.write_text(new)
            print("questions/README.md: index rebuilt")
        if pending_new != pending_old:
            PENDING.write_text(pending_new)
            print(f"{PENDING.relative_to(ROOT)}: count and decision packet rewritten")
    for p in problems:
        print(p)
    print(f"{len(questions)} questions, {len(problems)} problems")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
