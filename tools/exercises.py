#!/usr/bin/env python3
"""Build and score Predict batches for spec-only exercises (decision 0030).

Usage:
    tools/exercises.py predict PAGE OUT     # e.g. expressions/additive build/exercises/R-additive
    tools/exercises.py score OUT ANSWERS    # ANSWERS is JSON written by the reader

`predict` takes the tests that the rules on spec/PAGE.md cite, removes
their tidal- directives, and writes them to OUT as p01.cpp, p02.cpp, and
so on, in a shuffled order, with a README.md that tells the reader what
to answer. The key goes to build/exercise-keys/<name of OUT>.json, away
from OUT, so a reader given OUT does not see it.

The reader answers with a JSON object, one entry per program:

    {"p01": {"outcome": "reject", "rules": ["ADD-3"], "why": "..."}}

`outcome` is one of the OUTCOMES below. `score` compares the answers
with the key and prints a Markdown table of the wrong ones.
"""

import json
import random
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import matrix  # noqa: E402
import run_tests  # noqa: E402

ROOT = matrix.ROOT
SPEC = ROOT / "spec"
TESTS = ROOT / "tests"
KEYS = ROOT / "build" / "exercise-keys"

OUTCOMES = {
    "reject": "does not compile",
    "link-fail": "every translation unit compiles, but linking fails",
    "compiles": "compiles; the program is not run (it may have no main)",
    "runs": "compiles, runs, and exits with status 0",
    "run-fail": "compiles, but the run ends with a nonzero status",
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


def page_tests(page: str) -> list[str]:
    """Test names cited by the rules on one spec page, in page order."""
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
        key[pid] = {"test": name, "expect": sorted(expectations(test)), "rules": test.rules,
                    "main": has_main}
        build = " ".join(files)
        if test.flags:
            build += f", with {' '.join(test.flags)}"
        lines.append(f"| {pid} | {build} |")
    readme = [
        f"# Predict batch for {page}",
        "",
        f"Read spec/{page}.md and any other page under spec/. For each program",
        "below, predict what happens when it is built with",
        "`-std=c++23 -pedantic-errors` and, if it compiles and links, run.",
        "Do not compile the programs.",
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
        "spec does not settle a program, say so in `why` and give your best",
        "guess. Where the spec records that compilers differ on a program,",
        "the outcome of any of them counts.",
    ]
    (out / "README.md").write_text("\n".join(readme) + "\n")
    KEYS.mkdir(parents=True, exist_ok=True)
    (KEYS / f"{out.name}.json").write_text(json.dumps(key, indent=1) + "\n")
    print(f"{len(names)} programs in {out}; key in {KEYS / out.name}.json")
    return 0


def score(out: Path, answers_path: Path) -> int:
    key = json.loads((KEYS / f"{out.name}.json").read_text())
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
    if len(sys.argv) != 4 or sys.argv[1] not in {"predict", "score"}:
        print(__doc__)
        return 2
    if sys.argv[1] == "predict":
        return predict(sys.argv[2], Path(sys.argv[3]))
    return score(Path(sys.argv[2]), Path(sys.argv[3]))


if __name__ == "__main__":
    sys.exit(main())
