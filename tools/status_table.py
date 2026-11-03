#!/usr/bin/env python3
"""Regenerate the numbers in documentation/status.md from the files that hold them.

It also rewrites the agreement sentence in each page's Evidence segment
(tools/agreement.py).

The tool writes:
- the specification table. It lists every page that a section index
  includes, in site order, and replaces the lines below the table header.
  Its columns:
  - Rules, Classes, Tests, and Single-test rules come from the page.
  - Overrides counts the page's tests that carry a tidal-expect-<x> line.
  - Report mentions counts the lines of reports/clean-room/ that name one of
    the page's rules, leaving out the roadmap reports
    (reports/clean-room/roadmap*/).
  - Questions lists the questions the page names.
  - Similarity comes from the latest report under reports/similarity/ that
    checked any unit of the page. Reports are ordered by the date in their
    name, then by the time git first recorded them, then by name. A report
    that git has not recorded yet comes last in its date. The cell gives the number of the page's flagged
    units and the report. It also counts the page's rules and principles
    that the report does not check. A page that no report checked says
    "no report".
- the prose numbers: the baseline pass count from tests/RESULTS.md and,
  when some test files have no row there, the count of those files, the
  EDG override count, the question counts by status and the list of
  compiler-defect questions (tools/question_index.py), the
  *(referenced)* marker counts by topic, the tidal-error and sanitizer
  test counts, the front end's pass and check-pass counts
  (reports/clean-room/rust-frontend/conformance.tsv), and the name of the
  latest similarity report that checked every page.

It also writes the front-end row of the status table in
documentation/exercises.md (section 8): the latest round, from the
numbered reports in reports/clean-room/rust-frontend/, and the pass and
check-pass counts from conformance.tsv.

Each prose number sits in a fixed sentence. When a sentence is missing,
the tool reports a problem and leaves the text alone.
tools/check_specification.py fails when the page differs from what this
tool would write, and when the front-end row of exercises.md differs.

Usage:
    tools/status_table.py
"""

import argparse
import fnmatch
import re
import subprocess
import sys
import textwrap
from collections import Counter, defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import agreement  # noqa: E402
import frontend_check  # noqa: E402
import matrix  # noqa: E402
import question_index  # noqa: E402
import run_tests  # noqa: E402

ROOT = Path(__file__).resolve().parent.parent
SPEC = ROOT / "specification"
TESTS = ROOT / "tests"
STATUS = ROOT / "documentation" / "status.md"
EXERCISES = ROOT / "documentation" / "exercises.md"
RESULTS = TESTS / "RESULTS.md"
REPORTS = ROOT / "reports" / "clean-room"
SIMILARITY = ROOT / "reports" / "similarity"
CONFORMANCE = ROOT / "reports" / "clean-room" / "rust-frontend" / "conformance.tsv"
SECTIONS = ["foundations", "statements", "expressions", "declarations", "classes", "overloading", "templates", "exceptions"]
KINDS = ["required", "reject", "undefined", "unspecified", "impl-defined",
         "no-diagnostic", "conditionally-supported", "recommended"]
HEADER = ("| Section | Rules | Classes | Tests | Single-test rules | Overrides "
          "| Report mentions | Questions | Similarity |")
RULE_ID = re.compile(r"\b[A-Z]+-\d+\b")
REPORT_LINE = re.compile(r"^(ok  |FLAG) (.*?)\s+longest=\d+\s+overlap=[\d.]+$")
MARKER = re.compile(r"\*\(([^()*]*?)\breferenced\)\*")
# How the marker bullet names each topic.
TOPIC_NAMES = {"library": "the library", "preprocessor": "the preprocessor",
               "memory model": "the memory model"}
# The EDG row of the compiler table.
EDG = "edg-7.0"


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


def all_pages() -> list[Path]:
    return [p for section in SECTIONS for p in pages(SPEC / section / "index.md")]


def report_commit_times() -> dict[str, int]:
    """The commit time at which git first recorded each similarity report."""
    out = subprocess.run(["git", "-C", str(ROOT), "log", "--reverse", "--no-renames", "--diff-filter=A",
                          "--format=%ct", "--name-only", "--", "reports/similarity"],
                         capture_output=True, text=True).stdout
    times, when = {}, 0
    for line in out.splitlines():
        if line.isdigit():
            when = int(line)
        elif line:
            times.setdefault(Path(line).name, when)
    return times


class Similarity:
    """The committed similarity reports, oldest first."""

    def __init__(self):
        self.reports = []  # (stem, {label: flagged})
        added = report_commit_times()
        newest = float("inf")
        for f in sorted(SIMILARITY.glob("*.txt"),
                        key=lambda f: (f.stem[:10], added.get(f.name, newest), f.stem)):
            labels = {}
            for line in f.read_text().splitlines():
                if m := REPORT_LINE.match(line):
                    labels[m.group(2)] = m.group(1) == "FLAG"
            self.reports.append((f.stem, labels))

    @staticmethod
    def on_page(label: str, path: str, ids: set[str]) -> bool:
        """True when a report's unit label belongs to the page at path."""
        if label in ids:
            return True
        name = label.split(":", 1)[0]
        return name.endswith(".md") and (path == name or path.endswith("/" + name))

    def latest(self, path: str, ids: set[str]):
        """Return (stem, labels) of the latest report that checked the page, or None.

        A page with rules or principles counts as checked when a report has
        one of its IDs. A page without them counts when a label names its file.
        """
        for stem, labels in reversed(self.reports):
            if (ids & labels.keys()) if ids else any(self.on_page(l, path, ids) for l in labels):
                return stem, labels
        return None

    def cell(self, page: Path, ids: set[str]) -> str:
        path = str(page.relative_to(SPEC))
        found = self.latest(path, ids)
        if not found:
            return "no report"
        stem, labels = found
        flagged = sum(flag for label, flag in labels.items() if self.on_page(label, path, ids))
        text = f"{flagged or 'none'} flagged ([{stem}](../reports/similarity/{stem}.txt))"
        if missing := len(ids - labels.keys()):
            text += f"; {missing} IDs not checked"
        return text

    def covers_all(self, page_ids: dict[str, set[str]]) -> str | None:
        """The latest report that checked every page, by the test of latest()."""
        for stem, labels in reversed(self.reports):
            if all((ids & labels.keys()) if ids else any(self.on_page(l, path, ids) for l in labels)
                   for path, ids in page_ids.items()):
                return stem
        return None


def mention_index() -> dict[str, set[int]]:
    """Map each rule ID to the clean-room report lines that name it.

    The roadmap reports are left out, so a ranking of pages does not raise
    their counts.
    """
    index = defaultdict(set)
    n = 0
    for f in sorted(REPORTS.rglob("*.md")):
        if f.relative_to(REPORTS).parts[0].startswith("roadmap"):
            continue
        for line in f.read_text().splitlines():
            n += 1
            for rid in set(RULE_ID.findall(line)):
                index[rid].add(n)
    return index


def page_ids(src: str) -> tuple[set[str], set[str]]:
    """The rule IDs and the principle IDs a page defines."""
    rules = set(re.findall(r"^:::\{rule\} (\S+)$", src, flags=re.M))
    principles = set(re.findall(r"^:::\{principle\} (\S+)$", src, flags=re.M))
    return rules, principles


def row(page: Path, mentions: dict[str, set[int]], similarity: Similarity) -> str:
    src = page.read_text()
    title = src.splitlines()[0][2:].strip()
    kinds = Counter(re.findall(r"^:kind: (\S+)", src, flags=re.M))
    tests = set()
    single = 0
    for line in re.findall(r"^:tests: (.*)$", src, flags=re.M):
        tests.update(line.split())
        single += len(line.split()) == 1
    overrides = sum(1 for t in tests if (TESTS / t).exists()
                    and re.search(r"^// tidal-expect-", (TESTS / t).read_text(), flags=re.M))
    rules, principles = page_ids(src)
    lines = set().union(*(mentions.get(r, set()) for r in rules))
    classes = ", ".join(f"{kinds[k]} {k}" for k in KINDS if kinds[k])
    questions = ", ".join(sorted(set(re.findall(r"Q-\d{4}", src))))
    rel = page.relative_to(ROOT)
    return (f"| [{title}](../{rel}) | {sum(kinds.values())} | {classes} | {len(tests)} | {single} "
            f"| {overrides} | {len(lines)} | {questions} | {similarity.cell(page, rules | principles)} |")


def table(similarity: Similarity) -> list[str]:
    mentions = mention_index()
    rows = [row(p, mentions, similarity) for p in all_pages()]
    glossary = SPEC / "glossary.md"
    text = glossary.read_text()
    terms = len(re.findall(r"^\S.*$", text.split(":::{glossary}")[1].split(":::")[0], flags=re.M))
    rows.append(f"| [Glossary](../specification/glossary.md) | {terms} terms | | | | | | "
                f"| {similarity.cell(glossary, set())} |")
    return rows


def wrap(text: str, indent: str = "") -> str:
    return textwrap.fill(text, width=72, initial_indent=indent,
                         subsequent_indent=" " * len(indent), break_on_hyphens=False,
                         break_long_words=False)


def matrix_sentence() -> str:
    """The pass count, and the count of test files with no row in RESULTS.md when it is above zero."""
    text = RESULTS.read_text()
    baseline = len(re.findall(r"^- `[^`]+` \(baseline\):", text, flags=re.M))
    rows = [l for l in text.splitlines() if l.startswith("| `")]
    passing = sum(l.endswith("| pass |") for l in rows)
    sentence = f"The matrix has {baseline} baseline compilers, and {passing} of {len(rows)} tests pass on each."
    listed = {l.split("`")[1] for l in rows}
    files = [str(f.relative_to(TESTS)) for f in run_tests.test_files([TESTS])]
    if missing := sum(f not in listed for f in files):
        sentence += (f" The tree has {len(files)} test files, and {missing} of them have no row in "
                     "`tests/RESULTS.md`.")
    return sentence


def skipped(test: run_tests.Test, c: matrix.Compiler) -> bool:
    """True when the runner skips the test on the compiler (run_tests.outcome)."""
    return (any(fnmatch.fnmatchcase(f, u) for f in test.flags for u in c.unsupported_flags)
            or not all(f in test.flags for f in c.required_flags))


def test_counts() -> dict[str, int]:
    compilers = matrix.load()
    edg = next(c for c in compilers if c.id == EDG)
    edg_keys = {edg.id, edg.family} | ({edg.variant_of} if edg.variant_of else set())
    baseline = [c for c in compilers if c.role == "baseline"]
    counts = Counter()
    for path in run_tests.test_files([TESTS]):
        test = run_tests.parse_test(path)
        counts["edg"] += bool(edg_keys & test.overrides.keys())
        counts["errors"] += bool(test.errors)
        counts["sanitizer"] += any(f.startswith("-fsanitize") for f in test.flags)
        if test.errors:
            counts["no pattern"] += any(
                not skipped(test, c) and test.expect(c) in {"reject", "link-fail", "run-fail"}
                and test.error_pattern(c) is None for c in baseline)
    return counts


def questions_text() -> str:
    """The Open questions paragraph. tools/question_index.py reports bad question files."""
    questions, _ = question_index.load()
    by_status = Counter(q.status for q in questions)
    defects = ", ".join(q.id for q in questions if q.status == "compiler defect")
    settled = [q.id for q in questions if q.status == "settled"]
    open_ = by_status["provisional"] + by_status["open"]
    text = (f"The question index has {len(questions)} questions: {open_} open for opinions "
            f"({by_status['provisional']} provisional and {by_status['open']} open), "
            f"{by_status['compiler defect']} compiler defects, and {by_status['settled']} settled. "
            "Each open question has a provisional answer that the specification follows. "
            "A compiler-defect answer says that one compiler or an older version is wrong "
            f"({defects}).")
    if "Q-0009" in settled:
        text += " Q-0009 (license) is among the settled questions."
    return wrap(text + " See `questions/README.md`.")


def markers_text() -> str:
    topics = Counter()
    for f in SPEC.rglob("*.md"):
        for m in MARKER.finditer(f.read_text()):
            topics[" ".join(m.group(1).split())] += 1
    names = [f"{TOPIC_NAMES.get(t, t)} ({n})" for t, n in sorted(topics.items(), key=lambda x: (-x[1], x[0]))]
    listed = ", ".join(names[:-1]) + f", or {names[-1]}" if len(names) > 1 else "".join(names)
    total = sum(topics.values())
    return wrap(f"{total} *(referenced)* markers remain. Each names a topic with no page: {listed}.",
                indent="- ")


def frontend_counts() -> tuple[int, int, int]:
    """The front end's pass count, check-pass count, and test count from conformance.tsv."""
    status = [l.split("\t")[0] for l in CONFORMANCE.read_text().splitlines() if l and not l.startswith("#")]
    passing = sum(s in frontend_check.PASSING for s in status)
    checks = sum(frontend_check.rank(s) >= 1 for s in status)
    return passing, checks, len(status)


def frontend_text() -> str:
    passing, checks, total = frontend_counts()
    return f"clean-room front end passes {passing} of {total} tests, and {checks} pass their check."


EXERCISE_ROW = re.compile(r"^\| Front end \| [^|\n]* \| `reports/clean-room/rust-frontend/` \|$", re.M)
ROUND_REPORT = re.compile(r"^report-(\d+)[a-z]?\.md$")


def render_exercises(text: str) -> tuple[str, list[str]]:
    """Return exercises.md with the front-end row rewritten, and the problems found."""
    rounds = [int(m.group(1)) for f in CONFORMANCE.parent.iterdir() if (m := ROUND_REPORT.match(f.name))]
    passing, checks, total = frontend_counts()
    new = (f"| Front end | round {max(rounds)} done: {passing}/{total} pass, {checks} pass their check "
           "(`conformance.tsv`) | `reports/clean-room/rust-frontend/` |")
    text, n = EXERCISE_ROW.subn(lambda _: new, text)
    if n != 1:
        return text, [f"documentation/exercises.md: the front-end row matched {n} times; it must match once"]
    return text, []


def render(text: str) -> tuple[str, list[str]]:
    """Return status.md with every generated part rewritten, and the problems found."""
    problems = []
    similarity = Similarity()
    lines = text.split("\n")
    if HEADER not in lines:
        return text, ["documentation/status.md: the specification table header is missing"]
    start = lines.index(HEADER) + 2
    end = next(i for i in range(start, len(lines)) if not lines[i].startswith("|"))
    lines[start:end] = table(similarity)
    text = "\n".join(lines)

    counts = test_counts()
    ids = {}
    for p in all_pages():
        rules, principles = page_ids(p.read_text())
        ids[str(p.relative_to(SPEC))] = rules | principles
    ids["glossary.md"] = set()
    whole = similarity.covers_all(ids)
    replacements = [
        ("baseline pass count", r"The matrix has \d+ baseline compilers, and \d+ of \d+ tests\s+pass on each\."
         r"(?: The tree has \d+ test files, and \d+ of them have no row in\s+`tests/RESULTS\.md`\.)?",
         matrix_sentence()),
        ("EDG override count", r"per-compiler expectations on \d+ tests",
         f"per-compiler expectations on {counts['edg']} tests"),
        ("question counts", r"(?<=^## Open questions\n\n)(?:.+\n)+(?=\n)", questions_text() + "\n"),
        ("marker counts", r"^- \d+ \*\(referenced\)\* markers remain\.(?:.+\n)*?(?=\n|- )",
         markers_text() + "\n"),
        ("tidal-error count", r"^- \d+ tests carry `tidal-error` patterns\.",
         f"- {counts['errors']} tests carry `tidal-error` patterns."),
        ("missing pattern count", r"In \d+ tests, some compiler family\s+has no pattern",
         f"In {counts['no pattern']} tests, some compiler family has no pattern"),
        ("sanitizer count", r"sanitizers\s+\(\d+ tests, decision 0028\)",
         f"sanitizers ({counts['sanitizer']} tests, decision 0028)"),
        ("front end", r"clean-room front end passes \d+ of \d+ tests, and \d+ pass\s+their check\.",
         frontend_text()),
        ("similarity report", r"The latest similarity report that checked every page is\s+`[^`]*`\."
         r"|No similarity report has checked every page\.",
         f"The latest similarity report that checked every page is `reports/similarity/{whole}.txt`."
         if whole else "No similarity report has checked every page."),
    ]
    for name, pattern, new in replacements:
        text, n = re.subn(pattern, lambda _: new, text, flags=re.M)
        if n != 1:
            problems.append(f"documentation/status.md: the sentence with the {name} "
                            f"matched {n} times; it must match once")
    return text, problems


def main() -> int:
    argparse.ArgumentParser(description=__doc__.split("\n")[0]).parse_args()
    for p in all_pages():
        if agreement.rewrite(p):
            print(f"agreement sentence updated: {p.relative_to(ROOT)}")
    old = STATUS.read_text()
    new, problems = render(old)
    if new != old:
        STATUS.write_text(new)
        print("documentation/status.md: updated")
    old = EXERCISES.read_text()
    new, more = render_exercises(old)
    problems += more
    if new != old:
        EXERCISES.write_text(new)
        print("documentation/exercises.md: updated")
    for p in problems:
        print(p)
    print(f"{len(all_pages()) + 1} rows, {len(problems)} problems")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
