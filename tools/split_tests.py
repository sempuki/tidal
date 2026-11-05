"""The Split tests table in each page's Evidence segment.

The table has one row for each test that a page's `:tests:` lines cite
and that carries an override for a baseline compiler
(documentation/specification-format.md). The columns come from the test
headers: the rules, the base expectation, the expectation on each
family, and the questions that the test's notes name. A family cell
names each version when the versions of the family differ. A compiler
that skips the test does not count. tools/status_table.py rewrites the
tables, and tools/check_specification.py fails when one is out of date
or missing.
"""

import re
import textwrap
from pathlib import Path

import matrix
import run_tests

HEADING = "### Split tests"
HEADER = "| Test | Rules | Expected | Clang | GCC | EDG | Question |"
FAMILIES = [("clang", "Clang"), ("gcc", "GCC"), ("edg", "EDG")]
NAMES = {"clang-host": "host Clang", "gcc-host": "host GCC", "edg-7.0": "EDG 7.0",
         "edg-7.0-ilp32": "EDG 7.0 ILP32", "gcc-14": "GCC 14", "clang-18": "Clang 18",
         "gcc-16": "GCC 16", "clang-22": "Clang 22"}
# Newest version first.
ORDER = ["clang-22", "clang-18", "clang-host", "gcc-16", "gcc-14", "gcc-host", "edg-7.0", "edg-7.0-ilp32"]
QUESTION = re.compile(r"\bQ-\d{4}\b")


def skipped(test: run_tests.Test, c: matrix.Compiler) -> bool:
    """True when the runner skips the test on the compiler (run_tests.outcome)."""
    return (any(re.fullmatch(u.replace("*", ".*"), f) for f in test.flags for u in c.unsupported_flags)
            or not all(f in test.flags for f in c.required_flags))


def page_tests(page_text: str) -> list[str]:
    tests = []
    for line in re.findall(r"^:tests: (.*)$", page_text, flags=re.M):
        tests += [t for t in line.split() if t not in tests]
    return tests


def family_cell(test: run_tests.Test, compilers: list[matrix.Compiler]) -> str:
    """The expectation on one family, or each version's when they differ."""
    values = [(c, test.expect(c)) for c in compilers if not skipped(test, c)]
    if len({v for _, v in values}) <= 1:
        return values[0][1] if values else "skips"
    # Host compilers repeat a container version; name them only when they differ from it.
    shown = [(c, v) for c, v in values if not c.id.endswith("-host")]
    if len({v for _, v in shown}) <= 1:
        shown = values
    shown.sort(key=lambda cv: ORDER.index(cv[0].id) if cv[0].id in ORDER else len(ORDER))
    return "; ".join(f"{NAMES.get(c.id, c.id)}: {v}" for c, v in shown)


def rows(page_text: str, page_dir: str = "") -> list[str]:
    """The table rows. A test outside PAGE_DIR (a path under tests/) shows its path under tests/."""
    baseline = [c for c in matrix.load() if c.role == "baseline"]
    keys = {k for c in baseline for k in (c.id, c.family, c.variant_of) if k}
    out = []
    def shown(name: str) -> str:
        return name.removeprefix(page_dir + "/") if "/" not in name.removeprefix(page_dir + "/") else name

    for name in sorted(page_tests(page_text), key=shown):
        path = matrix.TESTS / name
        if not path.exists():
            continue
        test = run_tests.parse_test(path)
        if not keys & test.overrides.keys():
            continue
        cells = [family_cell(test, [c for c in baseline if c.family == fam]) for fam, _ in FAMILIES]
        questions = sorted({q for n in test.notes for q in QUESTION.findall(n)})
        question = ", ".join(f"{{question}}`{q}`" for q in questions)
        out.append(f"| `{shown(name)}` | {', '.join(test.rules)} | {test.default or 'none'} | "
                   + " | ".join(cells) + f" | {question} |".replace("|  |", "| |"))
    return out


# The Evidence parts that follow Split tests (documentation/specification-format.md).
LATER_PARTS = ["### Notes", "### Probe grids", "### Undefined rules", "### Sources"]
INTRO = ("Each row is a test that carries an override for a baseline compiler. "
         "The Expected column gives the outcome that the rules predict, and each "
         "family column gives the outcome that the test expects on that family.")
EDG_RUN = "EDG runs no programs, so its cell for a run test gives the result of the compile."


def page_dir(page: Path) -> str:
    """The test directory of a page under specification/, as a path under tests/."""
    spec = matrix.TESTS.parent / "specification"
    return str(page.resolve().relative_to(spec).with_suffix(""))


def render(page_text: str, directory: str) -> str:
    """The page with its Split tests table rewritten, added, or removed."""
    new = rows(page_text, directory)
    lines = page_text.split("\n")
    if HEADING in lines:
        start = lines.index(HEADING)
        head = lines.index(HEADER, start)
        end = head + 2
        while end < len(lines) and lines[end].startswith("|"):
            end += 1
        if new:
            lines[head + 2:end] = new
        else:
            # Drop the part: its heading, intro, and table, up to the next part.
            while end < len(lines) and not lines[end].startswith("#"):
                end += 1
            lines[start:end] = []
        return "\n".join(lines)
    if not new:
        return page_text
    intro = INTRO + (" " + EDG_RUN if any(r.split("|")[3].strip() in {"run", "run-fail"} for r in new) else "")
    part = [HEADING, "", *textwrap.wrap(intro, 72), "", HEADER, "|---|---|---|---|---|---|---|", *new, ""]
    evidence = lines.index("## Evidence")
    at = next((i for i in range(evidence, len(lines)) if lines[i] in LATER_PARTS), None)
    if at is None:
        while lines and lines[-1] == "":
            lines.pop()
        return "\n".join(lines + [""] + part)
    lines[at:at] = part
    return "\n".join(lines)


def current(page: Path) -> bool:
    text = page.read_text()
    return render(text, page_dir(page)) == text


def rewrite(page: Path) -> bool:
    """Rewrite the page's Split tests table. True if the page changed."""
    text = page.read_text()
    new = render(text, page_dir(page))
    if new == text:
        return False
    page.write_text(new)
    return True
