#!/usr/bin/env python3
"""Build and score Predict, Diagnose, and principle-use batches for clean-room exercises (decision 0030).

Usage:
    tools/exercises.py predict PAGE OUT     # e.g. expressions/additive build/exercises/R-additive
    tools/exercises.py diagnose PAGE OUT    # e.g. classes/virtual build/exercises/D-virtual
    tools/exercises.py principles OUT       # e.g. build/exercises/PU1
    tools/exercises.py score OUT ANSWERS    # ANSWERS is JSON written by the reader

`predict` takes the tests that the rules on specification/PAGE.md cite, removes
their tidal- directives, and writes them to OUT as p01.cpp, p02.cpp, and
so on, in a shuffled order, with a README.md that tells the reader what
to answer. It also writes a copy of specification/ to OUT/specification/
with every `:tests:` line removed, so test names do not show the
outcomes. The copy has no directive processing, so the generated table
of `undefined` rules is written into it as a Markdown table. The README
tells the reader to read that copy. The key goes to
build/exercise-keys/<name of OUT>.json, away from OUT, so a reader given
OUT does not see it.

The reader answers with a JSON object, one entry per program:

    {"p01": {"outcome": "reject", "rules": ["ADD-3"], "why": "..."}}

An answer can add a `principles` list, the principle IDs whose model
gave the outcome (documentation/style-guide.md, section 3):

    {"p01": {"outcome": "reject", "rules": ["ADD-3"], "principles": ["ADD-P1"], "why": "..."}}

`outcome` is one of the OUTCOMES below. `score` compares the answers
with the key and prints two numbers: the right outcomes, and how many of
those cite at least one rule that the test cites. It prints how many
answers name a principle, and for each principle the programs whose
answers name it, with each answer right or wrong. Each of these answers
is a use of the principle (decision 0056). It then prints a Markdown
table of the wrong outcomes.

`diagnose` takes the page's single-file tests that the specification and
GCC both reject and that carry a GCC error pattern. Each program comes with the
host g++ message that matches the pattern. The reader names the rule
and writes a fixed program pNN-fix.cpp:

    {"p01": {"rules": ["VIRT-3"], "why": "..."}}

`principles` builds a principle-use batch (decision 0056, item PU). It
selects each provisional question whose latest dated note says that the
question waits for a use of a principle (WAITS_FOR_USE below). The
principles it waits on are the first of these that gives any: principle
IDs in the note, the principles whose `:rules:` list a rule ID in the
note, principle IDs in the Affects line, and the principles of the rules
in the Affects line. The tool prints each question, its principles, and
where they came from. The programs are the single tests that the
question names above its Opinions and the tests whose tidal-note lines
cite the question, written as in `predict`. The copy of specification/
leaves out every rule block and every Evidence segment, so the reader
decides by the principles alone. An answer adds `part`, the part of the
model that decides the program, and can give the outcome `not-decided`.
The key gives each program the questions it belongs to. `score` on such
a batch prints, per question, whether every program has the right
outcome and whether each answer names a principle that the question
waits on.

`score` on a D- batch counts a rule as right when it is one the test
cites, and builds each fix with g++ (and runs it when it has a main).
"""

import json
import random
import re
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import matrix  # noqa: E402
import run_tests  # noqa: E402
import undefined_index  # noqa: E402

ROOT = matrix.ROOT
SPEC = ROOT / "specification"
TESTS = ROOT / "tests"
KEYS = ROOT / "build" / "exercise-keys"

OUTCOMES = {
    "reject": "does not compile",
    "link-fail": "every translation unit compiles, but linking fails",
    "compiles": "compiles; the program is not run (it may have no main, or its run may never end)",
    "runs": "compiles, runs, and exits with status 0",
    "run-fail": "compiles, but the run ends with a nonzero status",
    "undefined": "compiles, but running it has undefined behavior, so no outcome is promised; when the build flags include a sanitizer that stops the operation, answer `run-fail`",
    "depends": "the compilers differ, as the specification records; say which in `why`",
}
# A test expectation and the answers that match it.
MATCHES = {
    "reject": {"reject"},
    "link-fail": {"link-fail"},
    "accept": {"compiles", "runs"},
    "run": {"runs"},
    "run-fail": {"run-fail"},
}

RULE = re.compile(r"^:::\{rule\} (\S+)$")
TESTS_LINE = re.compile(r"^:tests: (.*)$")
MAIN = re.compile(r"\bint\s+main\s*\(")
INCLUDE = re.compile(r'^\s*#\s*include\s*"([^"]+)"', re.M)


def rule_classes() -> dict[str, str]:
    """The class of every rule in the specification."""
    classes, rule = {}, None
    for f in SPEC.rglob("*.md"):
        for line in f.read_text().splitlines():
            if m := RULE.match(line):
                rule = m.group(1)
            elif rule and line.startswith(":kind: "):
                classes[rule] = line.removeprefix(":kind: ").strip()
                rule = None
    return classes


RULE_CLASS = rule_classes()


def page_tests(page: str) -> list[str]:
    """Test names cited by the rules on one specification page, in page order."""
    names, in_rule = [], False
    for line in (SPEC / f"{page}.md").read_text().splitlines():
        if RULE.match(line):
            in_rule = True
        elif line == ":::":
            in_rule = False
        elif in_rule and (m := TESTS_LINE.match(line)):
            names += [n for n in m.group(1).split() if n not in names]
    return names


def strip(text: str) -> str:
    """Drop tidal- directive lines and comment-only lines."""
    return "".join(l for l in text.splitlines(keepends=True) if not l.lstrip().startswith("//"))


def expectations(test: run_tests.Test) -> set[str]:
    """Every outcome some baseline compiler is expected to give."""
    found = set()
    for c in matrix.load():
        if c.role == "baseline":
            e = test.overrides.get(c.id) or test.overrides.get(c.family) or test.default
            if e is not None:
                found.add(e)
    return found


def copy_specification(out: Path) -> None:
    """Copy specification/ to OUT/specification/ without its :tests: lines.

    The table of `undefined` rules is a directive that only the site
    renders, so the copy gets the table as Markdown in its place.
    """
    for f in sorted(SPEC.rglob("*")):
        target = out / "specification" / f.relative_to(SPEC)
        if f.is_dir():
            continue
        target.parent.mkdir(parents=True, exist_ok=True)
        if f.suffix == ".md":
            text = "".join(l for l in f.read_text().splitlines(keepends=True)
                           if not TESTS_LINE.match(l.rstrip("\n")))
            if undefined_index.DIRECTIVE in text:
                text = text.replace(undefined_index.DIRECTIVE, undefined_index.markdown(SPEC, f))
            target.write_text(text)
        else:
            target.write_bytes(f.read_bytes())


def included_headers(sources: list[Path]) -> list[Path]:
    """The headers beside the sources that a quoted #include names, and theirs."""
    found, todo = [], list(sources)
    while todo:
        source = todo.pop()
        for name in INCLUDE.findall(source.read_text()):
            header = (source.parent / name).resolve()
            if header.is_file() and header not in found:
                found.append(header)
                todo.append(header)
    return found


def write_program(out: Path, pid: str, name: str) -> tuple[dict, str]:
    """Write one test to OUT as program PID; return its key entry and README row."""
    test = run_tests.parse_test(TESTS / name)
    files = [f"{pid}.cpp"]
    (out / files[0]).write_text(strip(test.path.read_text()))
    for j, extra in enumerate(test.sources, 2):
        files.append(f"{pid}-{j}.cpp")
        (out / files[-1]).write_text(strip(extra.read_text()))
    for header in included_headers([test.path, *test.sources]):
        target = out / header.relative_to(test.path.parent.resolve())
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(strip(header.read_text()))
    has_main = any(MAIN.search(f.read_text()) for f in [test.path, *test.sources])
    undefined = any(RULE_CLASS.get(r) == "undefined" for r in test.rules)
    entry = {"test": name, "expect": sorted(expectations(test)), "rules": test.rules,
             "main": has_main, "undefined": undefined}
    build = " ".join(files)
    if test.flags:
        build += f", with {' '.join(test.flags)}"
    return entry, f"| {pid} | {build} |"


def predict(page: str, out: Path) -> int:
    names = page_tests(page)
    if not names:
        print(f"no tests found for {page}")
        return 1
    random.Random(page).shuffle(names)  # neighbors in page order share a rule
    out.mkdir(parents=True, exist_ok=False)
    copy_specification(out)
    key, lines = {}, []
    for i, name in enumerate(names, 1):
        pid = f"p{i:02d}"
        key[pid], row = write_program(out, pid, name)
        lines.append(row)
    readme = [
        f"# Predict batch for {page}",
        "",
        f"Read specification/{page}.md and any other page under specification/,",
        "in the copy in this directory. The copy leaves out the lists of tests.",
        "For each program below, predict what happens when it is built with",
        "`-std=c++23 -pedantic-errors` and no `-O` flag, and, if it compiles",
        "and links, run. Do not compile the programs. The target is x86-64",
        "Linux, unless the flags include `-m32`. The baseline compilers section",
        "of specification/index.md says which compilers run which programs.",
        "",
        "| Program | Files and flags |",
        "|---|---|",
        *lines,
        "",
        "Answer with one JSON object, one entry per program:",
        "",
        '    {"p01": {"outcome": "reject", "rules": ["ADD-3"], "why": "one sentence"}}',
        "",
        "`outcome` is one of:",
        "",
        *[f"- `{k}`: {v}" for k, v in OUTCOMES.items()],
        "",
        "`rules` lists the Tidal rule IDs that decide the outcome. When the",
        "specification does not settle a program, say so in `why` and give your best",
        "guess. Where the specification records that compilers differ on a program,",
        "answer `depends`, or the outcome of any one of them. The compilers",
        "are the eight baseline compilers, old versions included.",
        "",
        "An answer can also list `principles`: the IDs of the principles",
        "(`:::{principle}` blocks) whose model gave you the outcome. Name a",
        "principle only when you reached the outcome from its prose, tables,",
        "or diagrams for this program. Leave the list out when a rule decided",
        "the outcome:",
        "",
        '    {"p01": {"outcome": "reject", "rules": ["ADD-3"], "principles": ["ADD-P1"], "why": "one sentence"}}',
        "",
        "Some programs break a rule that needs no diagnostic, or a rule whose",
        "behavior is undefined during translation, such as a constant evaluation",
        "that reaches an operation of class `undefined`. For these, answer with",
        "what the compilers do, and cite the rule: `compiles` or `runs` when",
        "the specification records that every compiler accepts the program,",
        "`reject` when every compiler rejects it, and `depends` when they differ.",
        "",
        "A program whose run has undefined behavior is answered `undefined`.",
        "Such a program's test checks only that it compiles, so `compiles` is",
        "also right for it. The specification's notes say this as \"checks only",
        "that the program compiles\".",
    ]
    (out / "README.md").write_text("\n".join(readme) + "\n")
    KEYS.mkdir(parents=True, exist_ok=True)
    (KEYS / f"{out.name}.json").write_text(json.dumps(key, indent=1) + "\n")
    print(f"{len(names)} programs in {out}; key in {KEYS / out.name}.json")
    return 0


QUESTIONS = ROOT / "questions"
PRINCIPLE = re.compile(r"^:::\{principle\} (\S+)$")
RULE_END = re.compile(r"^:::$")
# The start of a dated note, such as "Kept open on 2026-11-04" or "Used on 2026-11-04".
DATED = re.compile(r"(?:^|(?<=[.)] ))[A-Z][a-z]+(?: [a-z]+)? on \d{4}-\d{2}-\d{2}\b")
# A note that keeps a question open until a reader or a reviewer uses a principle.
WAITS_FOR_USE = re.compile(r"\bwaits? for (?:the next use|a use|a reader or a reviewer to use"
                           r"|the next review or reader batch)")
PRINCIPLE_ID = re.compile(r"\b[A-Z]+-P\d+\b")
RULE_ID = re.compile(r"\b[A-Z]+-\d+\b")
TEST_PATH = re.compile(r"\btests/([\w/.+-]+\.cpp)\b")
NOT_DECIDED = "not-decided"


def principle_rules() -> dict[str, list[str]]:
    """For each rule, the principles whose `:rules:` list names it."""
    owners: dict[str, list[str]] = {}
    for f in sorted(SPEC.rglob("*.md")):
        principle = None
        for line in f.read_text().splitlines():
            if m := PRINCIPLE.match(line):
                principle = m.group(1)
            elif principle and line.startswith(":rules: "):
                for rule in line.removeprefix(":rules: ").split():
                    owners.setdefault(rule, []).append(principle)
            elif RULE_END.match(line):
                principle = None
    return owners


def latest_note(text: str) -> str | None:
    """The last dated note above the Opinions segment of a question."""
    body = text.split("\n## Opinions")[0]
    found = None
    for para in body.split("\n\n"):
        para = " ".join(para.split())
        for m in DATED.finditer(para):
            found = para[m.start():]
    return found


def waited_principles(note: str, affects: str, owners: dict[str, list[str]]) -> tuple[list[str], str]:
    """The principles a waiting question waits on, and where they came from.

    The first of these that gives any principle wins: principle IDs in the
    note; the principles of the rule IDs in the note; principle IDs in the
    Affects line; the principles of the rules in the Affects line.
    """
    def of_rules(text: str) -> list[str]:
        found = []
        for rule in RULE_ID.findall(text):
            for p in owners.get(rule, []):
                if p not in found:
                    found.append(p)
        return found

    for source, ids in [("principle IDs in the note", PRINCIPLE_ID.findall(note)),
                        ("principles of the rules in the note", of_rules(note)),
                        ("principle IDs in the Affects line", PRINCIPLE_ID.findall(affects)),
                        ("principles of the rules in the Affects line", of_rules(affects))]:
        ids = list(dict.fromkeys(ids))
        if ids:
            return ids, source
    return [], "no principle found"


def note_citations() -> dict[str, set[str]]:
    """For each question ID, the tests whose tidal-note lines cite it."""
    cited: dict[str, set[str]] = {}
    for f in TESTS.rglob("*.cpp"):
        for line in f.read_text().splitlines():
            if not line.startswith("//"):
                break
            if line.startswith("// tidal-note:"):
                for q in re.findall(r"\bQ-\d{4}\b", line):
                    cited.setdefault(q, set()).add(str(f.relative_to(TESTS)))
    return cited


def usable_test(name: str) -> bool:
    """A single test that a batch can show: not an extra unit, not a whole program."""
    return ((TESTS / name).is_file() and not name.endswith(run_tests.EXTRA_SUFFIX)
            and not name.startswith("programs/"))


def waiting_questions() -> list[dict]:
    """The open questions whose latest dated note waits for a use of a principle."""
    owners, cited, chosen = principle_rules(), note_citations(), []
    for f in sorted(QUESTIONS.glob("Q-[0-9][0-9][0-9][0-9]*.md")):
        text = f.read_text()
        if "**Status:** provisional" not in text:
            continue
        note = latest_note(text)
        if not note or not (m := WAITS_FOR_USE.search(note)):
            continue
        qid = f.name[:6]
        affects = next((l for l in text.splitlines() if l.startswith("**Affects:**")), "")
        principles, source = waited_principles(note, affects, owners)
        body = text.split("\n## Opinions")[0]
        named = list(dict.fromkeys(TEST_PATH.findall(body)))
        tests = [t for t in named if usable_test(t)]
        tests += sorted(t for t in cited.get(qid, set()) if usable_test(t) and t not in tests)
        chosen.append({"question": qid, "principles": principles, "source": source,
                       "phrase": m.group(0), "tests": tests,
                       "skipped": [t for t in named if not usable_test(t)]})
    return chosen


def principles_only(text: str) -> str:
    """A page without its rule blocks, its Evidence segment, and segments left empty."""
    kept, in_rule = [], False
    for line in text.splitlines(keepends=True):
        if RULE.match(line.rstrip("\n")):
            in_rule = True
        elif in_rule:
            in_rule = not RULE_END.match(line.rstrip("\n"))
        else:
            kept.append(line)
    # Split at level-2 headings, drop Evidence and segments with nothing left.
    segments, current = [], []
    for line in kept:
        if line.startswith("## ") and current:
            segments.append(current)
            current = []
        current.append(line)
    segments.append(current)
    out = []
    for seg in segments:
        heading = seg[0].strip() if seg[0].startswith("## ") else None
        if heading == "## Evidence":
            continue
        if heading and not "".join(seg[1:]).strip():
            continue
        out += seg
    return re.sub(r"\n{3,}", "\n\n", "".join(out))


def copy_principles(out: Path) -> None:
    """Copy specification/ to OUT/specification/ with only principles, syntax, examples, and the glossary."""
    for f in sorted(SPEC.rglob("*")):
        if f.is_dir():
            continue
        target = out / "specification" / f.relative_to(SPEC)
        target.parent.mkdir(parents=True, exist_ok=True)
        if f.suffix == ".md":
            target.write_text(principles_only(f.read_text()))
        else:
            target.write_bytes(f.read_bytes())


def principles(out: Path) -> int:
    chosen = waiting_questions()
    print("Selected: each provisional question whose latest dated note says it waits for a use")
    print(f"(the note matches {WAITS_FOR_USE.pattern!r}).")
    print()
    print("| Question | Waits on | Taken from | Tests | Phrase in the note |")
    print("|---|---|---|---|---|")
    for q in chosen:
        print(f"| {q['question']} | {' '.join(q['principles']) or 'none'} | {q['source']} | "
              f"{len(q['tests'])} | {q['phrase']} |")
    for q in chosen:
        if q["skipped"]:
            print(f"{q['question']}: left out {' '.join(q['skipped'])} (whole programs or extra units)")
        if not q["tests"]:
            print(f"{q['question']}: no test to show, so the batch leaves the question out")
        if not q["principles"]:
            print(f"{q['question']}: no principle found, so the batch leaves the question out")
    chosen = [q for q in chosen if q["tests"] and q["principles"]]
    names: dict[str, dict[str, list[str]]] = {}  # test -> question -> principles
    for q in chosen:
        for t in q["tests"]:
            names.setdefault(t, {})[q["question"]] = q["principles"]
    order = sorted(names)
    random.Random(out.name).shuffle(order)
    out.mkdir(parents=True, exist_ok=False)
    copy_principles(out)
    key, lines = {}, []
    for i, name in enumerate(order, 1):
        pid = f"p{i:02d}"
        entry, row = write_program(out, pid, name)
        entry["questions"] = names[name]
        key[pid] = entry
        lines.append(row)
    waited = sorted({p for q in chosen for p in q["principles"]})
    readme = [
        f"# Principle-use batch {out.name}",
        "",
        "The copy of the specification in this directory holds only the",
        "principles, the syntax, the examples, and the glossary. It leaves out",
        "every rule and every Evidence segment. Read only this copy.",
        "",
        "For each program below, predict what happens when it is built with",
        "`-std=c++23 -pedantic-errors` and no `-O` flag, and, if it compiles",
        "and links, run. Do not compile the programs. The target is x86-64",
        "Linux, unless the flags include `-m32`. The baseline compilers section",
        "of specification/index.md says which compilers run which programs.",
        "",
        "Decide each outcome from the principles alone. A principle's model is",
        "its prose, its tables, and its diagrams. Do not use what you know of",
        "C++ from elsewhere. When no principle gives the outcome, answer",
        f"`{NOT_DECIDED}`. That answer is a finding about the principles, and",
        "it is as useful as a right outcome.",
        "",
        "The batch reads these principles, among others that a program may need:",
        "",
        ", ".join(f"`{p}`" for p in waited) + ".",
        "",
        "| Program | Files and flags |",
        "|---|---|",
        *lines,
        "",
        "Answer with one JSON object, one entry per program:",
        "",
        '    {"p01": {"outcome": "reject", "principles": ["ADD-P1"], "part": "the second row of the table", "why": "one sentence"}}',
        "",
        "`outcome` is one of:",
        "",
        *[f"- `{k}`: {v}" for k, v in OUTCOMES.items()],
        f"- `{NOT_DECIDED}`: no principle gives the outcome; say in `why` what the principles leave open",
        "",
        "`principles` lists the IDs of the principles (`:::{principle}` blocks)",
        "whose model gave the outcome. `part` names the part of that model that",
        "decides the program: a sentence, a list item, a table row, or a part of",
        f"a diagram. For `{NOT_DECIDED}`, list the principles you tried and say",
        "in `part` what each one does not say.",
        "",
        "A program whose run has undefined behavior is answered `undefined`.",
        "Such a program's test checks only that it compiles, so `compiles` is",
        "also right for it.",
    ]
    (out / "README.md").write_text("\n".join(readme) + "\n")
    KEYS.mkdir(parents=True, exist_ok=True)
    (KEYS / f"{out.name}.json").write_text(json.dumps(key, indent=1) + "\n")
    print()
    print(f"{len(chosen)} questions and {len(key)} programs in {out}; key in {KEYS / out.name}.json")
    return 0


def score_principles(key: dict, answers: dict) -> int:
    """Score a principle-use batch per question."""
    right = 0
    per_question: dict[str, dict] = {}
    uses: dict[str, list[str]] = {}
    rows = []
    for pid, k in key.items():
        a = answers.get(pid, {})
        got = a.get("outcome")
        ok = got != NOT_DECIDED and outcome_right(k, got)
        right += ok
        named = set(a.get("principles", []))
        for principle in sorted(named):
            uses.setdefault(principle, []).append(f"{pid} {'right' if ok else 'wrong'}")
        for qid, waited in k["questions"].items():
            q = per_question.setdefault(qid, {"waits": waited, "programs": [], "right": 0, "named": 0,
                                             "undecided": 0})
            q["programs"].append(pid)
            q["right"] += ok
            q["named"] += ok and bool(named & set(waited))
            q["undecided"] += got == NOT_DECIDED
        if not ok:
            rows.append(f"| {pid} | {k['test']} | {' '.join(k['questions'])} | {', '.join(k['expect'])} | "
                        f"{got} | {' '.join(a.get('principles', []))} | "
                        f"{a.get('why', '').replace('|', '/')} |")
    print(f"{right}/{len(key)} predictions match the tests")
    print()
    print("A question's answer is reached when every program of the question has the right")
    print("outcome. A use supports the answer when, in addition, every program's answer names")
    print("a principle that the question waits on.")
    print()
    print("| Question | Waits on | Programs | Right | Name a waited principle | Not decided | Answer reached | Use supports the answer |")
    print("|---|---|---|---|---|---|---|---|")
    for qid in sorted(per_question):
        q = per_question[qid]
        n = len(q["programs"])
        reached = q["right"] == n
        print(f"| {qid} | {' '.join(q['waits'])} | {' '.join(q['programs'])} | {q['right']}/{n} | "
              f"{q['named']}/{n} | {q['undecided']} | {'yes' if reached else 'no'} | "
              f"{'yes' if reached and q['named'] == n else 'no'} |")
    if uses:
        print()
        print("| Principle | Answers that name it |")
        print("|---|---|")
        for principle in sorted(uses):
            print(f"| {principle} | {', '.join(uses[principle])} |")
    if rows:
        print()
        print("| Program | Test | Questions | Expected | Answer | Principles named | Reason given |")
        print("|---|---|---|---|---|---|---|")
        print("\n".join(rows))
    return 0


GCC_FLAGS = ["-std=c++23", "-pedantic-errors"]


def gcc_message(source: Path, pattern: str) -> str | None:
    """The first g++ error line that matches the test's pattern."""
    r = subprocess.run(["g++", *GCC_FLAGS, "-fsyntax-only", source.name], cwd=source.parent,
                       capture_output=True, text=True)
    for line in r.stderr.splitlines():
        if re.search(pattern, line):
            return line
    return None


def diagnose(page: str, out: Path) -> int:
    chosen = []
    for name in page_tests(page):
        test = run_tests.parse_test(TESTS / name)
        pattern = test.errors.get("gcc") or test.errors.get("")
        expect = test.overrides.get("gcc-host") or test.overrides.get("gcc") or test.default
        # Only programs that the specification rejects. A program that only
        # GCC rejects (an override) breaks no rule, so it has no answer.
        if (pattern and expect == "reject" and test.default == "reject"
                and not test.sources and not test.flags):
            chosen.append((name, test, pattern))
    if not chosen:
        print(f"no tests with a GCC error pattern for {page}")
        return 1
    random.Random(page).shuffle(chosen)
    out.mkdir(parents=True, exist_ok=False)
    key, lines, n = {}, [], 0
    for name, test, pattern in chosen:
        pid = f"p{n + 1:02d}"
        src = out / f"{pid}.cpp"
        src.write_text(strip(test.path.read_text()))
        message = gcc_message(src, pattern)
        if message is None:  # the stripped program no longer gives the message
            src.unlink()
            continue
        n += 1
        key[pid] = {"test": name, "rules": test.rules, "main": bool(MAIN.search(src.read_text()))}
        lines += [f"## {pid}", "", "```", message, "```", ""]
    readme = [
        f"# Diagnose batch for {page}",
        "",
        f"Read specification/{page}.md and any other page under specification/. Each program",
        "below fails to compile with `g++ -std=c++23 -pedantic-errors`, and the",
        "message under its name is the first matching line g++ prints.",
        "",
        "For each program, name the Tidal rule that the program breaks, and",
        "write a fixed program `pNN-fix.cpp` beside it that keeps the program's",
        "intent and compiles. A fix of a program with `main` must also run and",
        "exit with status 0. You may compile and run your fixes.",
        "",
        "Answer with one JSON object, one entry per program:",
        "",
        '    {"p01": {"rules": ["VIRT-3"], "why": "one sentence"}}',
        "",
        *lines,
    ]
    (out / "README.md").write_text("\n".join(readme))
    KEYS.mkdir(parents=True, exist_ok=True)
    (KEYS / f"{out.name}.json").write_text(json.dumps(key, indent=1) + "\n")
    print(f"{n} programs in {out}; key in {KEYS / out.name}.json")
    return 0


def fix_works(fix: Path, has_main: bool) -> bool:
    exe = fix.with_suffix("")
    build = subprocess.run(["g++", *GCC_FLAGS, fix.name, "-o", exe.name] if has_main
                           else ["g++", *GCC_FLAGS, "-c", fix.name, "-o", exe.name + ".o"],
                           cwd=fix.parent, capture_output=True)
    if build.returncode or not has_main:
        return build.returncode == 0
    try:
        return subprocess.run([str(exe)], capture_output=True, timeout=10).returncode == 0
    except subprocess.TimeoutExpired:
        return False


def score_diagnose(out: Path, key: dict, answers: dict) -> int:
    right, fixed, rows = 0, 0, []
    for pid, k in key.items():
        a = answers.get(pid, {})
        ok = bool(set(a.get("rules", [])) & set(k["rules"]))
        fix = out / f"{pid}-fix.cpp"
        works = fix.exists() and fix_works(fix, k["main"])
        right += ok
        fixed += works
        if not (ok and works):
            rows.append(f"| {pid} | {k['test']} | {' '.join(k['rules'])} | "
                        f"{' '.join(a.get('rules', []))} | {'yes' if works else 'no'} | "
                        f"{a.get('why', '').replace('|', '/')} |")
    print(f"{right}/{len(key)} rules match the tests; {fixed}/{len(key)} fixes build and run")
    if rows:
        print()
        print("| Program | Test | Rules of the test | Rules cited | Fix works | Reason given |")
        print("|---|---|---|---|---|---|")
        print("\n".join(rows))
    return 0


def outcome_right(k: dict, got: str | None) -> bool:
    """Whether a predicted outcome matches the key entry K."""
    ok = any(got in MATCHES[e] for e in k["expect"])
    # The runner links a program with no main only when its test
    # expects a link failure, which the reader cannot see. So for
    # such a program, compiling and failing to link both count.
    if not k.get("main", True) and got in {"compiles", "link-fail"}:
        ok = ok or bool({"accept", "link-fail"} & set(k["expect"]))
    # "depends" is right when the baseline compilers are expected to
    # differ. "undefined" is right when a cited rule is undefined and no
    # compiler rejects the program, so the behavior shows only at run time.
    if got == "depends":
        ok = len(k["expect"]) > 1
    if got == "undefined":
        ok = k.get("undefined", False) and "reject" not in k["expect"]
    return ok


def score(out: Path, answers_path: Path) -> int:
    key = json.loads((KEYS / f"{out.name}.json").read_text())
    if out.name.startswith("D-"):
        return score_diagnose(out, key, json.loads(answers_path.read_text()))
    if key and all("questions" in k for k in key.values()):
        return score_principles(key, json.loads(answers_path.read_text()))
    answers = json.loads(answers_path.read_text())
    right, cited, rows = 0, 0, []
    uses: dict[str, list[str]] = {}
    for pid, k in key.items():
        a = answers.get(pid, {})
        got = a.get("outcome")
        ok = outcome_right(k, got)
        for principle in a.get("principles", []):
            uses.setdefault(principle, []).append(f"{pid} {'right' if ok else 'wrong'}")
        right += ok
        cited += ok and bool(set(a.get("rules", [])) & set(k["rules"]))
        if not ok:
            rows.append(f"| {pid} | {k['test']} | {', '.join(k['expect'])} | {got} | "
                        f"{' '.join(a.get('rules', []))} | {a.get('why', '').replace('|', '/')} |")
    print(f"{right}/{len(key)} predictions match the tests; "
          f"{cited} of the {right} right outcomes cite one of the test's rules")
    named = sum(1 for pid in key if answers.get(pid, {}).get("principles"))
    print(f"{named} of {len(key)} answers name a principle")
    if uses:
        print()
        print("| Principle | Answers that name it |")
        print("|---|---|")
        for principle in sorted(uses):
            print(f"| {principle} | {', '.join(uses[principle])} |")
    if rows:
        print()
        print("| Program | Test | Expected | Answer | Rules cited | Reason given |")
        print("|---|---|---|---|---|---|")
        print("\n".join(rows))
    return 0


def main() -> int:
    if len(sys.argv) == 3 and sys.argv[1] == "principles":
        return principles(Path(sys.argv[2]))
    if len(sys.argv) != 4 or sys.argv[1] not in {"predict", "diagnose", "score"}:
        print(__doc__)
        return 2
    if sys.argv[1] == "predict":
        return predict(sys.argv[2], Path(sys.argv[3]))
    if sys.argv[1] == "diagnose":
        return diagnose(sys.argv[2], Path(sys.argv[3]))
    return score(Path(sys.argv[2]), Path(sys.argv[3]))


if __name__ == "__main__":
    sys.exit(main())
