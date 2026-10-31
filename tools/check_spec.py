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
- a tidal-expect-<x> or tidal-error-<x> names no compiler id or family, or a
  tidal-error is on a test that no compiler is expected to reject, fail to
  link, or fail to run,
- a *(TOPIC referenced)* marker names no topic, or a topic that is not in
  REFERENCED_TOPICS,
- a page's agreement sentence is out of date (run tools/status_table.py),
- a grammar nonterminal is defined in more than one production, or used
  and not defined (decision 0040),
- a page of spec/ or guides/ refers with {rule} or {principle} to an ID
  that no page defines, or a spec page links to a guide.

guides/ holds the tutorials and how-to guides, which are not part of the
specification (decision 0045). The question, marker, and reference checks
cover the guides too.
"""

import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from run_tests import TESTS, parse_test, test_files  # noqa: E402

ROOT = Path(__file__).resolve().parent.parent
SPEC = ROOT / "spec"
GUIDES = ROOT / "guides"
CLASSES = {
    "required", "reject", "no-diagnostic", "impl-defined",
    "unspecified", "undefined", "recommended", "conditionally-supported",
    "withdrawn",
}
HEADING = re.compile(r"^### ([A-Z]+-\d+) \(([a-z-]+)\)\s*$")
TESTS_LINE = re.compile(r"^Tests:\s*(.*)$")
DIRECTIVE = re.compile(r"^:::\{rule\}\s+([A-Z]+-\d+)\s*$")
PRINCIPLE = re.compile(r"^:::\{principle\}\s+([A-Z]+-P\d+)\s*$")
PRULES = re.compile(r"^:(rules|exceptions):\s*(.*)$")
OPTION = re.compile(r"^:(kind|tests):\s*(.*)$")
SEGMENT = re.compile(r"^## (.+?)\s*$")
# Which classes each segment may hold (docs/style-guide.md, decision 0016).
# Classes whose rules a reject test can cite (decision 0034).
MAY_REJECT = {"reject", "undefined", "recommended", "impl-defined",
              "conditionally-supported", "unspecified", "no-diagnostic"}
SEGMENT_KINDS = {
    "Constraints": {"required", "reject", "withdrawn"},
    "Runtime Behavior": {"required", "withdrawn"},
    "Undefined Behavior": {"undefined", "withdrawn"},
    "Implementation Requirements": {"impl-defined", "unspecified", "no-diagnostic", "conditionally-supported", "withdrawn"},
    "Recommended Practice": {"recommended", "withdrawn"},
}


def parse_spec():
    """Return {rule_id: (spec_file, class, [test paths])}."""
    rules = {}
    errors = []
    principles = {}  # id -> (file, line, [rule ids])
    for f in sorted(SPEC.rglob("*.md")):
        current = None
        segment = None
        directive = None
        principle = None
        for n, line in enumerate(f.read_text().splitlines(), 1):
            if m := SEGMENT.match(line):
                segment = m.group(1)
            if m := PRINCIPLE.match(line):
                principle = m.group(1)
                if principle in principles:
                    errors.append(f"{f}:{n}: duplicate principle {principle}")
                principles[principle] = (f, n, [])
                if segment != "Principles":
                    errors.append(f"{f}:{n}: {principle} is outside the Principles segment")
                continue
            if principle:
                if m := PRULES.match(line):
                    f0, n0, ids = principles[principle]
                    principles[principle] = (f0, n0, ids + m.group(2).split())
                elif line.startswith(":::"):
                    principle = None
                continue
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
    for pid, (f, n, cited) in principles.items():
        if not cited:
            errors.append(f"{f}:{n}: {pid} lists no rules; a principle needs evidence")
        for rid in cited:
            if rid not in rules:
                errors.append(f"{f}:{n}: {pid} cites unknown rule {rid}")
            elif rules[rid][0] != f:
                errors.append(f"{f}:{n}: {pid} cites {rid}, which is on another page")
    pages_with_principles = {f for f, _, _ in principles.values()}
    cited = {rid for _, _, ids in principles.values() for rid in ids}
    for rid, (f, cls, _) in rules.items():
        if f in pages_with_principles and rid not in cited and cls != "withdrawn":
            errors.append(f"{f}: {rid} is not cited by any principle")
    return rules, errors


# Topics that pages mention but no page specifies. A marker
# *(TOPIC referenced)* must name one of them. When a topic gets a page,
# remove it here, and the check lists every marker to replace with a link.
REFERENCED_TOPICS = {
    "class properties",      # the full trivial and standard-layout lists
    "concepts",
    "coroutines",
    "extended floating-point types",
    "library",
    "memory model",
    "templates",
    "threads",
}
MARKER = re.compile(r"\*\(([^()*]*?)\breferenced\)\*")


def check_markers(errors: list[str]) -> None:
    for f in sorted(SPEC.rglob("*.md")):
        if re.search(r"\{doc\}`[^`]*guides/", f.read_text()):
            errors.append(f"{f.relative_to(ROOT)}: a spec page links to a guide (decision 0045)")
    for f in sorted([*SPEC.rglob("*.md"), *GUIDES.rglob("*.md")]):
        text = f.read_text()
        for m in MARKER.finditer(text):
            topic = " ".join(m.group(1).split())
            line = text.count("\n", 0, m.start()) + 1
            where = f"{f.relative_to(ROOT)}:{line}"
            if not topic:
                errors.append(f"{where}: a *(referenced)* marker must name its topic")
            elif topic not in REFERENCED_TOPICS:
                errors.append(f"{where}: marker names {topic!r}, which is not a referenced topic")


PRODUCTION = re.compile(r"^\s+([A-Za-z_][\w-]*)\s*:(.*)$")
NONTERMINAL = re.compile(r"`([A-Za-z_][\w-]*)`")


def check_grammar(errors: list[str]) -> None:
    defined, used = {}, set()
    for f in sorted(SPEC.rglob("*.md")):
        inside = False
        for n, line in enumerate(f.read_text().splitlines(), 1):
            if line.startswith(".. productionlist::"):
                inside = True
                continue
            if inside and (not line.startswith(" ") or not line.strip()):
                inside = False
            if not inside:
                continue
            body = line
            if (m := PRODUCTION.match(line)) and not line.lstrip().startswith(":"):
                defined.setdefault(m.group(1), []).append(f"{f.relative_to(ROOT)}:{n}")
                body = m.group(2)
            used.update(NONTERMINAL.findall(body))
    for name, places in sorted(defined.items()):
        if len(places) > 1:
            errors.append(f"{places[1]}: nonterminal {name} is also defined at {places[0]}")
    for name in sorted(used - defined.keys()):
        errors.append(f"grammar nonterminal {name} is used and not defined")


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
        for key in test.errors:
            if key and key not in known:
                errors.append(f"{test.name}: tidal-error-{key} names no compiler or family")
        if test.errors and not {"reject", "link-fail", "run-fail"} & {test.default, *test.overrides.values()}:
            errors.append(f"{test.name}: tidal-error on a test that no compiler is expected to reject, fail to link, or fail to run")
        if test.default == "reject" and all(rid in rules for rid in test.rules) and not (
                {rules[rid][1] for rid in test.rules} & MAY_REJECT):
            errors.append(f"{test.name}: expects reject, but cites no rule of a class that allows it (decision 0034)")
        for rid in test.rules:
            if rid not in rules and rid.rsplit("-", 1)[0] not in prefixes:
                warnings.append(f"{test.name}: names {rid}; no {rid.rsplit('-', 1)[0]} rules are drafted yet")
            elif rid not in rules:
                errors.append(f"{test.name}: names unknown rule {rid}")
            elif rid not in listed.get(test.name, set()):
                errors.append(f"{test.name}: names {rid}, but {rid} doesn't list it")

    # The prefix table on rules.md is for readers of the Markdown sources,
    # where the rule index directive does not render.
    pages = {(rid.rsplit("-", 1)[0], str(f.relative_to(SPEC))) for rid, (f, _, _) in rules.items()}
    table = set(re.findall(r"^\| `([A-Z]+)` \| `([^`]+)` \|$", (SPEC / "rules.md").read_text(), re.M))
    for prefix, page in sorted(pages - table):
        errors.append(f"rules.md: the prefix table lacks {prefix} ({page})")
    for prefix, page in sorted(table - pages):
        errors.append(f"rules.md: the prefix table lists {prefix} ({page}), which has no rules")

    questions = {q.name[:6] for q in (ROOT / "questions").glob("Q-[0-9][0-9][0-9][0-9]-*.md")}
    for f in [*SPEC.rglob("*.md"), *GUIDES.rglob("*.md"), *TESTS.rglob("*.cpp"),
              *(ROOT / "docs").rglob("*.md")]:
        for ref in set(re.findall(r"\bQ-\d{4}\b", f.read_text())):
            if ref not in questions:
                errors.append(f"{f.relative_to(ROOT)}: refers to missing question {ref}")

    check_markers(errors)
    principles = set()
    for f in SPEC.rglob("*.md"):
        principles.update(re.findall(r"^:::\{principle\} (\S+)$", f.read_text(), re.M))
    for f in sorted(SPEC.rglob("*.md")):
        if re.search(r"\{doc\}`[^`]*guides/", f.read_text()):
            errors.append(f"{f.relative_to(ROOT)}: a spec page links to a guide (decision 0045)")
    for f in sorted([*SPEC.rglob("*.md"), *GUIDES.rglob("*.md")]):
        text = f.read_text()
        for role, ref in re.findall(r"\{(rule|principle)\}`([^`<]+?)`", text):
            if ref not in (rules if role == "rule" else principles):
                errors.append(f"{f.relative_to(ROOT)}: {{{role}}} refers to {ref}, which no page defines")
    import agreement
    for f in sorted(SPEC.rglob("*.md")):
        if not agreement.current(f.read_text()):
            errors.append(f"{f.relative_to(ROOT)}: the agreement sentence is out of date; run tools/status_table.py")
    check_grammar(errors)

    for w in warnings:
        print("warning: " + w)
    for e in errors:
        print(e)
    print(f"{len(rules)} rules, {len(errors)} problems")
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main())
