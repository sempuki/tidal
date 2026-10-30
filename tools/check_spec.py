#!/usr/bin/env python3
"""Check that spec rules and tests refer to each other correctly.

Fails when:
- a rule ID appears twice,
- a rule has an unknown class,
- a rule has no Tests line, or lists a test file that doesn't exist,
- a test names a rule that doesn't exist,
- a test is not listed by any of the rules it names.
"""

import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from run_tests import TESTS, parse_test  # noqa: E402

ROOT = Path(__file__).resolve().parent.parent
SPEC = ROOT / "spec"
CLASSES = {
    "required", "reject", "no-diagnostic", "impl-defined",
    "unspecified", "undefined", "withdrawn",
}
HEADING = re.compile(r"^### ([A-Z]+-\d+) \(([a-z-]+)\)\s*$")
TESTS_LINE = re.compile(r"^Tests:\s*(.*)$")


def parse_spec():
    """Return {rule_id: (spec_file, class, [test paths])}."""
    rules = {}
    errors = []
    for f in sorted(SPEC.rglob("*.md")):
        current = None
        for n, line in enumerate(f.read_text().splitlines(), 1):
            if m := HEADING.match(line):
                rid, cls = m.groups()
                if rid in rules:
                    errors.append(f"{f}:{n}: duplicate rule {rid}")
                if cls not in CLASSES:
                    errors.append(f"{f}:{n}: {rid} has unknown class {cls}")
                rules[rid] = (f, cls, None)
                current = rid
            elif line.startswith("#"):
                current = None
            elif current and (m := TESTS_LINE.match(line)):
                tests = [t.strip() for t in m.group(1).split(",") if t.strip()]
                rules[current] = (rules[current][0], rules[current][1], tests)
    return rules, errors


def main() -> int:
    rules, errors = parse_spec()
    listed = {}
    for rid, (f, cls, tests) in rules.items():
        if cls == "withdrawn":
            continue
        if not tests:
            errors.append(f"{f}: {rid} has no Tests line")
            continue
        for t in tests:
            if not (TESTS / t).exists():
                errors.append(f"{f}: {rid} lists missing test {t}")
            listed.setdefault(t, set()).add(rid)

    for path in sorted(TESTS.rglob("*.cpp")):
        test = parse_test(path)
        for rid in test.rules:
            if rid not in rules:
                errors.append(f"{test.name}: names unknown rule {rid}")
            elif rid not in listed.get(test.name, set()):
                errors.append(f"{test.name}: names {rid}, but {rid} doesn't list it")

    for e in errors:
        print(e)
    print(f"{len(rules)} rules, {len(errors)} problems")
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main())
