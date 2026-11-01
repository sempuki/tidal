#!/usr/bin/env python3
"""Give each placeholder question the next free number.

A writer files a new question as questions/Q-NEW-<slug>.md, with the
first line "# Q-NEW-<slug>: <question>", and cites it as Q-NEW-<slug> in
pages, tests, and other questions. The slug uses lowercase letters,
digits, and hyphens. The integrator runs this tool once the writers are
done. The tool:

1. gives the placeholders the next free numbers, in the order of their slugs,
2. renames questions/Q-NEW-<slug>.md to questions/Q-NNNN-<slug>.md,
3. rewrites every Q-NEW-<slug> under specification/, guides/, tests/,
   questions/, and documentation/,
4. rebuilds the index in questions/README.md (tools/question_index.py).

The tool changes nothing when a reference names a placeholder that has no
file, or when a numbered file already uses the slug's new name.

Usage:
    tools/number_questions.py [--dry-run]
"""

import argparse
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import question_index  # noqa: E402

ROOT = Path(__file__).resolve().parent.parent
QUESTIONS = ROOT / "questions"
DIRS = ["specification", "guides", "tests", "questions", "documentation"]
SLUG = r"[a-z0-9]+(?:-[a-z0-9]+)*"
PLACEHOLDER = re.compile(rf"\bQ-NEW-({SLUG})")
NUMBERED = re.compile(r"^Q-(\d{4})-")


def text_files() -> list[Path]:
    files = []
    for d in DIRS:
        for f in sorted((ROOT / d).rglob("*")):
            if f.is_file() and "__pycache__" not in f.parts:
                try:
                    f.read_text()
                except UnicodeDecodeError:
                    continue
                files.append(f)
    return files


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--dry-run", action="store_true", help="print the numbers and change nothing")
    args = ap.parse_args()

    placeholders = {}
    for f in sorted(QUESTIONS.glob("Q-NEW-*.md")):
        m = re.fullmatch(rf"Q-NEW-({SLUG})\.md", f.name)
        if not m:
            print(f"{f.relative_to(ROOT)}: a placeholder file is named Q-NEW-<slug>.md, "
                  "with lowercase letters, digits, and hyphens")
            return 1
        placeholders[m.group(1)] = f
    if not placeholders:
        print("no placeholder questions")
        return 0

    used = [int(m.group(1)) for f in QUESTIONS.glob("Q-*.md") if (m := NUMBERED.match(f.name))]
    first = max(used, default=0) + 1
    numbers = {slug: f"Q-{n:04d}" for n, slug in enumerate(sorted(placeholders), first)}

    files = text_files()
    problems = []
    for f in files:
        for slug in set(PLACEHOLDER.findall(f.read_text())):
            if slug not in placeholders:
                problems.append(f"{f.relative_to(ROOT)}: Q-NEW-{slug} has no file questions/Q-NEW-{slug}.md")
    for slug, qid in numbers.items():
        target = QUESTIONS / f"{qid}-{slug}.md"
        if target.exists():
            problems.append(f"{target.relative_to(ROOT)} exists already")
    if problems:
        print("\n".join(problems))
        print(f"{len(problems)} problems; nothing changed")
        return 1

    for slug, qid in numbers.items():
        print(f"Q-NEW-{slug} -> {qid}")
    if args.dry_run:
        return 0

    changed = 0
    for f in files:
        text = f.read_text()
        new = PLACEHOLDER.sub(lambda m: numbers[m.group(1)], text)
        if new != text:
            f.write_text(new)
            changed += 1
    for slug, qid in numbers.items():
        placeholders[slug].rename(QUESTIONS / f"{qid}-{slug}.md")

    questions, problems = question_index.load()
    readme = question_index.README.read_text()
    question_index.README.write_text(question_index.render(readme, questions))
    for p in problems:
        print(p)
    print(f"{len(numbers)} questions numbered, {changed} files rewritten, index rebuilt")
    return 0


if __name__ == "__main__":
    sys.exit(main())
