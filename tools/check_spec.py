#!/usr/bin/env python3
"""Check that spec rules and tests refer to each other correctly.

Fails when:
- a rule ID appears twice,
- a rule has an unknown class,
- a rule has no Tests line, or lists a test file that doesn't exist,
- a test names a rule that doesn't exist (only a warning while no rule with
  that prefix exists, which is the state of a section between pipeline
  steps 3 and 6),
- a test is not listed by any of the rules it names,
- a spec, test, or doc refers to a question (Q-NNNN) with no file in questions/,
- a tidal-expect-<x> override names no compiler id or family.
"""

import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from run_tests import TESTS, parse_test, test_files  # noqa: E402

ROOT = Path(__file__).resolve().parent.parent
SPEC = ROOT / "spec"
CLASSES = {
    "required", "reject", "no-diagnostic", "impl-defined",
    "unspecified", "undefined", "recommended", "withdrawn",
}
HEADING = re.compile(r"^### ([A-Z]+-\d+) \(([a-z-]+)\)\s*$")
TESTS_LINE = re.compile(r"^Tests:\s*(.*)$")
DIRECTIVE = re.compile(r"^:::\{rule\}\s+([A-Z]+-\d+)\s*$")
OPTION = re.compile(r"^:(kind|tests):\s*(.*)$")
SEGMENT = re.compile(r"^## (.+?)\s*$")
# Which classes each segment may hold (docs/style-guide.md, decision 0016).
SEGMENT_KINDS = {
    "Constraints": {"required", "reject", "withdrawn"},
    "Dynamic Semantics": {"required", "withdrawn"},
    "Undefined Behavior": {"undefined", "withdrawn"},
    "Implementation Requirements": {"impl-defined", "unspecified", "no-diagnostic", "withdrawn"},
    "Recommended Practice": {"recommended", "withdrawn"},
}


def parse_spec():
    """Return {rule_id: (spec_file, class, [test paths])}."""
    rules = {}
    errors = []
    for f in sorted(SPEC.rglob("*.md")):
        current = None
        segment = None
        directive = None
        for n, line in enumerate(f.read_text().splitlines(), 1):
            if m := SEGMENT.match(line):
                segment = m.group(1)
            if m := DIRECTIVE.match(line):
                directive = m.group(1)
                if directive in rules:
                    errors.append(f"{f}:{n}: duplicate rule {directive}")
                rules[directive] = (f, None, None)
                if segment not in SEGMENT_KINDS:
                    errors.append(f"{f}:{n}: {directive} is outside a rule segment ({segment})")
                continue
            if directive:
                if m := OPTION.match(line):
                    key, value = m.groups()
                    f0, cls, tests = rules[directive]
                    if key == "kind":
                        cls = value
                        if cls not in CLASSES:
                            errors.append(f"{f}:{n}: {directive} has unknown class {cls}")
                        elif segment in SEGMENT_KINDS and cls not in SEGMENT_KINDS[segment]:
                            errors.append(f"{f}:{n}: {directive} ({cls}) doesn't belong in {segment}")
                    else:
                        tests = value.split()
                    rules[directive] = (f0, cls, tests)
                    continue
                if line.startswith(":::"):
                    if rules[directive][1] is None:
                        errors.append(f"{f}:{n}: {directive} has no :kind:")
                    directive = None
                continue
            if HEADING.match(line):
                errors.append(f"{f}:{n}: old rule heading; use a rule directive (docs/style-guide.md)")
                continue
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

    prefixes = {rid.rsplit("-", 1)[0] for rid in rules}
    warnings = []
    import matrix
    known = {c.id for c in matrix.load()} | {c.family for c in matrix.load()}
    for path in test_files([TESTS]):
        test = parse_test(path)
        for key in test.overrides:
            if key not in known:
                errors.append(f"{test.name}: tidal-expect-{key} names no compiler or family")
        for rid in test.rules:
            if rid not in rules and rid.rsplit("-", 1)[0] not in prefixes:
                warnings.append(f"{test.name}: names {rid}; no {rid.rsplit('-', 1)[0]} rules are drafted yet")
            elif rid not in rules:
                errors.append(f"{test.name}: names unknown rule {rid}")
            elif rid not in listed.get(test.name, set()):
                errors.append(f"{test.name}: names {rid}, but {rid} doesn't list it")

    questions = {q.name[:6] for q in (ROOT / "questions").glob("Q-[0-9][0-9][0-9][0-9]-*.md")}
    for f in [*SPEC.rglob("*.md"), *TESTS.rglob("*.cpp"), *(ROOT / "docs").rglob("*.md")]:
        for ref in set(re.findall(r"\bQ-\d{4}\b", f.read_text())):
            if ref not in questions:
                errors.append(f"{f.relative_to(ROOT)}: refers to missing question {ref}")

    for w in warnings:
        print("warning: " + w)
    for e in errors:
        print(e)
    print(f"{len(rules)} rules, {len(errors)} problems")
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main())
