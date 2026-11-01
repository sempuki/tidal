#!/usr/bin/env python3
"""Build and score Predict and Diagnose batches for clean-room exercises (decision 0030).

Usage:
    tools/exercises.py predict PAGE OUT     # e.g. expressions/additive build/exercises/R-additive
    tools/exercises.py diagnose PAGE OUT    # e.g. classes/virtual build/exercises/D-virtual
    tools/exercises.py score OUT ANSWERS    # ANSWERS is JSON written by the reader

`predict` takes the tests that the rules on specification/PAGE.md cite, removes
their tidal- directives, and writes them to OUT as p01.cpp, p02.cpp, and
so on, in a shuffled order, with a README.md that tells the reader what
to answer. The key goes to build/exercise-keys/<name of OUT>.json, away
from OUT, so a reader given OUT does not see it.

The reader answers with a JSON object, one entry per program:

    {"p01": {"outcome": "reject", "rules": ["ADD-3"], "why": "..."}}

`outcome` is one of the OUTCOMES below. `score` compares the answers
with the key and prints a Markdown table of the wrong ones.

`diagnose` takes the page's single-file tests that the specification and
GCC both reject and that carry a GCC error pattern. Each program comes with the
host g++ message that matches the pattern. The reader names the rule
and writes a fixed program pNN-fix.cpp:

    {"p01": {"rules": ["VIRT-3"], "why": "..."}}

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

ROOT = matrix.ROOT
SPEC = ROOT / "specification"
TESTS = ROOT / "tests"
KEYS = ROOT / "build" / "exercise-keys"

OUTCOMES = {
    "reject": "does not compile",
    "link-fail": "every translation unit compiles, but linking fails",
    "compiles": "compiles; the program is not run (it may have no main)",
    "runs": "compiles, runs, and exits with status 0",
    "run-fail": "compiles, but the run ends with a nonzero status",
    "undefined": "compiles, but running it has undefined behavior, so no outcome is promised",
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


def predict(page: str, out: Path) -> int:
    names = page_tests(page)
    if not names:
        print(f"no tests found for {page}")
        return 1
    random.Random(page).shuffle(names)  # neighbors in page order share a rule
    out.mkdir(parents=True, exist_ok=False)
    key, lines = {}, []
    for i, name in enumerate(names, 1):
        test = run_tests.parse_test(TESTS / name)
        pid = f"p{i:02d}"
        files = [f"{pid}.cpp"]
        (out / files[0]).write_text(strip(test.path.read_text()))
        for j, extra in enumerate(test.sources, 2):
            files.append(f"{pid}-{j}.cpp")
            (out / files[-1]).write_text(strip(extra.read_text()))
        has_main = any(MAIN.search(f.read_text()) for f in [test.path, *test.sources])
        undefined = any(RULE_CLASS.get(r) == "undefined" for r in test.rules)
        key[pid] = {"test": name, "expect": sorted(expectations(test)), "rules": test.rules,
                    "main": has_main, "undefined": undefined}
        build = " ".join(files)
        if test.flags:
            build += f", with {' '.join(test.flags)}"
        lines.append(f"| {pid} | {build} |")
    readme = [
        f"# Predict batch for {page}",
        "",
        f"Read specification/{page}.md and any other page under specification/. For each program",
        "below, predict what happens when it is built with",
        "`-std=c++23 -pedantic-errors` and no `-O` flag, and, if it compiles",
        "and links, run. Do not compile the programs.",
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
        "Some programs break a rule that needs no diagnostic, or a rule whose",
        "behavior is undefined during translation. For these, answer with",
        "what the compilers do, and cite the rule.",
    ]
    (out / "README.md").write_text("\n".join(readme) + "\n")
    KEYS.mkdir(parents=True, exist_ok=True)
    (KEYS / f"{out.name}.json").write_text(json.dumps(key, indent=1) + "\n")
    print(f"{len(names)} programs in {out}; key in {KEYS / out.name}.json")
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


def score(out: Path, answers_path: Path) -> int:
    key = json.loads((KEYS / f"{out.name}.json").read_text())
    if out.name.startswith("D-"):
        return score_diagnose(out, key, json.loads(answers_path.read_text()))
    answers = json.loads(answers_path.read_text())
    right, rows = 0, []
    for pid, k in key.items():
        a = answers.get(pid, {})
        got = a.get("outcome")
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
        right += ok
        if not ok:
            rows.append(f"| {pid} | {k['test']} | {', '.join(k['expect'])} | {got} | "
                        f"{' '.join(a.get('rules', []))} | {a.get('why', '').replace('|', '/')} |")
    print(f"{right}/{len(key)} predictions match the tests")
    if rows:
        print()
        print("| Program | Test | Expected | Answer | Rules cited | Reason given |")
        print("|---|---|---|---|---|---|")
        print("\n".join(rows))
    return 0


def main() -> int:
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
