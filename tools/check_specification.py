#!/usr/bin/env python3
"""Check that specification rules and tests refer to each other correctly.

Fails when:
- a rule ID appears twice,
- a rule has an unknown class,
- a rule has no Tests line, or lists a test file that doesn't exist,
- a test names a rule that doesn't exist (only a warning while no rule with
  that prefix exists, which is the state of a section between pipeline
  steps 3 and 6),
- a rule's Tests line lists a test whose tidal-rules header does not cite
  the rule,
- a test is not listed by any of the rules it names. A program test under
  tests/programs/ is the exception (decision 0054). It names rules that exist
  and are not withdrawn, and no rule lists it, so it never counts as a
  rule's test or as the one test that decides a rule (decision 0044),
- a specification page, test, or document refers to a question (Q-NNNN) with no file in questions/,
- a test that some compiler is expected to fail to link cites no no-diagnostic
  rule (TR-4: no class requires a diagnostic at link time),
- a test has no base tidal-expect line, or a tidal-allows-edg line names a
  rule that the test does not cite, a rule whose class allows no second
  answer, or sits on a test whose EDG answer is the base answer,
- a tidal-expect-<x> or tidal-error-<x> names no compiler id or family, or a
  tidal-error is on a test that no compiler is expected to reject, fail to
  link, or fail to run,
- a *(TOPIC referenced)* marker names no topic, or a topic that is not in
  REFERENCED_TOPICS,
- a page's agreement sentence is out of date (run tools/status_table.py),
- a page's Split tests table is out of date or missing (run tools/status_table.py),
- a front-end report names a finding with no row in
  reports/clean-room/rust-frontend/ledger.md, or another report under
  reports/clean-room/ names a finding with no row in
  reports/clean-room/ledger.md,
- a generated number in documentation/status.md is out of date (run
  tools/status_table.py),
- the front-end row of documentation/exercises.md, or its sentence on the
  pages whose only Predict batches showed test names, is out of date (run
  tools/status_table.py),
- the index in questions/README.md is out of date (run
  tools/question_index.py), or a question's answer starts with an option
  number,
- a grammar nonterminal is defined in more than one production, or used
  and not defined (decision 0040),
- a page of specification/ or guides/ refers with {rule} or {principle} to an ID
  that no page defines, or a specification page links to a guide,
- a rule that the working tree adds, or whose class it changes, relative to
  HEAD rests on thin evidence (see check_thin_evidence),
- a page's Evidence segment has a ### heading that the Evidence format does
  not name, or names two parts out of order (documentation/specification-format.md,
  section "Evidence"),
- a page's Evidence names a question and has no Divergences part (the older
  form), or names a question that has no line of its own in Divergences.

Warns for every rule that rests on thin evidence: one test, or only tests that
carry an override for a newest compiler.

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
SPEC = ROOT / "specification"
GUIDES = ROOT / "guides"
# Program tests (decision 0054): whole programs from the Write exercises.
# Each names the rules its report gives, and no rule lists it.
PROGRAMS = "programs/"
CLASSES = {
    "required", "reject", "no-diagnostic", "impl-defined",
    "unspecified", "undefined", "recommended", "conditionally-supported",
    "withdrawn",
}
HEADING = re.compile(r"^### ([A-Z]+-\d+) \(([a-z-]+)\)\s*$")
DIRECTIVE = re.compile(r"^:::\{rule\}\s+([A-Z]+-\d+)\s*$")
PRINCIPLE = re.compile(r"^:::\{principle\}\s+([A-Z]+-P\d+)\s*$")
PRULES = re.compile(r"^:(rules|exceptions):\s*(.*)$")
OPTION = re.compile(r"^:(kind|tests):\s*(.*)$")
SEGMENT = re.compile(r"^## (.+?)\s*$")
# Which classes each segment may hold (documentation/style-guide.md, decision 0016).
# Classes whose rules a reject test can cite (decision 0034).
MAY_REJECT = {"reject", "undefined", "recommended", "impl-defined",
              "conditionally-supported", "unspecified", "no-diagnostic"}
# Classes whose rule can allow EDG's answer where it differs from the base
# expectation (tidal-allows-edg, documentation/specification-format.md).
ALLOWS_EDG = {"impl-defined", "unspecified", "no-diagnostic", "conditionally-supported"}
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
                errors.append(f"{f}:{n}: old rule heading; use a rule directive (documentation/style-guide.md)")
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
    "coroutines",
    "extended floating-point types",
    "library",
    "memory model",
    "preprocessor",          # #if evaluation and macro replacement
    "threads",
}
MARKER = re.compile(r"\*\(([^()*]*?)\breferenced\)\*")


def check_markers(errors: list[str]) -> None:
    for f in sorted(SPEC.rglob("*.md")):
        if re.search(r"\{doc\}`[^`]*guides/", f.read_text()):
            errors.append(f"{f.relative_to(ROOT)}: a specification page links to a guide (decision 0045)")
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


FRONTEND = ROOT / "reports" / "clean-room" / "rust-frontend"
LEDGER_STATES = {"commit", "question", "text", "out of scope", "open", "planned"}
EXERCISE_LEDGER = ROOT / "reports" / "clean-room" / "ledger.md"
FINDING_HEAD = re.compile(r"^(?:#{2,4} |\*\*)((?:[A-Z][A-Z0-9]*)-\d+)\b", re.M)


def check_ledger(errors: list[str]) -> None:
    """Each finding that a front-end report names has one row in the ledger (decision 0057)."""
    ledger = FRONTEND / "ledger.md"
    rows = {}
    for line in ledger.read_text().splitlines():
        if m := re.match(r"^\| (G\d+) \| [^|]+ \| ([^|]+?) \|", line):
            if m.group(1) in rows:
                errors.append(f"{ledger.relative_to(ROOT)}: {m.group(1)} has two rows")
            rows[m.group(1)] = m.group(2)
            if m.group(2) not in LEDGER_STATES:
                errors.append(f"{ledger.relative_to(ROOT)}: {m.group(1)} has an unknown state {m.group(2)!r}")
    for report in sorted(FRONTEND.glob("report*.md")):
        for g in re.findall(r"^#+ (G\d+)[.:]", report.read_text(), re.M):
            if g not in rows:
                errors.append(f"{report.relative_to(ROOT)}: finding {g} has no row in {ledger.relative_to(ROOT)}")
    # Every other exercise finding has a row in the exercise ledger (decision 0058).
    exercise_rows = {}
    for line in EXERCISE_LEDGER.read_text().splitlines():
        if m := re.match(r"^\| ([A-Z][A-Z0-9]*-\d+) \| [^|]+ \| ([^|]+?) \|", line):
            exercise_rows[m.group(1)] = m.group(2)
            if m.group(2) not in LEDGER_STATES:
                errors.append(f"{EXERCISE_LEDGER.relative_to(ROOT)}: {m.group(1)} has an unknown state {m.group(2)!r}")
    for report in sorted((ROOT / "reports" / "clean-room").rglob("*.md")):
        rel = report.relative_to(ROOT)
        if report == EXERCISE_LEDGER or "roadmap" in str(rel) or "rust-frontend" in str(rel):
            continue
        for fid in sorted(set(FINDING_HEAD.findall(report.read_text()))):
            if fid not in exercise_rows:
                errors.append(f"{rel}: finding {fid} has no row in {EXERCISE_LEDGER.relative_to(ROOT)}")


def check_records(errors: list[str], warnings: list[str]) -> None:
    """The status page and the question index must match what their tools write."""
    import question_index
    import status_table
    questions, problems = question_index.load()
    errors.extend(problems)
    readme = question_index.README.read_text()
    if question_index.render(readme, questions) != readme:
        errors.append("questions/README.md: the index is out of date; run tools/question_index.py")
    status = status_table.STATUS.read_text()
    new, problems = status_table.render(status)
    errors.extend(problems)
    if new != status:
        errors.append("documentation/status.md: a generated number is out of date; run tools/status_table.py")
    exercises = status_table.EXERCISES.read_text()
    new, problems = status_table.render_exercises(exercises)
    errors.extend(problems)
    if new != exercises:
        errors.append("documentation/exercises.md: the front-end row or the section 7 sentence is out of date; "
                      "run tools/status_table.py")
    for f in [*SPEC.rglob("*.md"), *GUIDES.rglob("*.md"), *TESTS.rglob("*.cpp"),
              *(ROOT / "documentation").rglob("*.md"), *(ROOT / "questions").glob("*.md")]:
        if re.search(r"\bQ-NEW-[a-z0-9]", f.read_text()):
            warnings.append(f"{f.relative_to(ROOT)}: names a placeholder question ID; "
                            "the integrator runs tools/number_questions.py")


# The subsections of an Evidence segment, in order (documentation/specification-format.md).
EVIDENCE_PARTS = ["Divergences", "Split tests", "Notes", "Probe grids", "Undefined rules", "Sources"]


def check_evidence(errors: list[str], warnings: list[str]) -> None:
    """Evidence subsections come from EVIDENCE_PARTS, in that order, and each
    question that Evidence names has a line in its Divergences part."""
    import status_table
    for f in sorted(SPEC.rglob("*.md")):
        text = status_table.evidence(f.read_text())
        where = f.relative_to(ROOT)
        headings = re.findall(r"^### (.+?)\s*$", text, re.M)
        for h in headings:
            if h not in EVIDENCE_PARTS:
                errors.append(f"{where}: Evidence has a part {h!r} that the Evidence format does not name")
        known = [h for h in headings if h in EVIDENCE_PARTS]
        order = [EVIDENCE_PARTS.index(h) for h in known]
        if order != sorted(set(order)) or len(order) != len(set(order)):
            errors.append(f"{where}: Evidence parts out of order: {', '.join(known)}; "
                          f"the order is {', '.join(EVIDENCE_PARTS)}")
        named = set(re.findall(r"\bQ-\d{4}\b", text))
        if named and "Divergences" not in headings:
            errors.append(f"{where}: Evidence names a question and has no Divergences part (the older form)")
            continue
        if m := re.search(r"^### Divergences\n(.*?)(?=^### |\Z)", text, re.S | re.M):
            lines = set(re.findall(r"^- \{question\}`(Q-\d{4})`", m.group(1), re.M))
            for q in sorted(named - lines):
                errors.append(f"{where}: Evidence names {q} and Divergences has no line for it")


def newest_overrides() -> set[str]:
    """The override keys that apply to a newest baseline compiler.

    An override names a family (clang, gcc, edg) or a compiler id. A
    family key applies to the newest compiler of the family. An id key
    counts unless the compiler is a report compiler or an older version
    of its family, such as gcc-14 beside gcc-16 (decision 0055, item TE).
    """
    import matrix
    compilers = matrix.load()
    version = {}
    for c in compilers:
        if m := re.fullmatch(rf"{re.escape(c.family)}-(\d+)(?:\.\d+)?(?:-.+)?", c.id):
            version[c.id] = int(m.group(1))
    top = {}
    for cid, v in version.items():
        family = next(c.family for c in compilers if c.id == cid)
        top[family] = max(top.get(family, v), v)
    keys = {c.family for c in compilers}
    for c in compilers:
        if c.role != "baseline":
            continue
        if c.id in version and version[c.id] < top[c.family]:
            continue
        keys.add(c.id)
    return keys


def thin_evidence(rules: dict) -> dict[str, str]:
    """{rule id: reason} for each rule that rests on one test, or only on tests
    that carry an override for a newest compiler."""
    newest = newest_overrides()
    thin = {}
    for rid, (_, cls, tests) in rules.items():
        if cls == "withdrawn" or not tests:
            continue
        if len(tests) == 1:
            thin[rid] = "rests on one test"
            continue
        if all((TESTS / t).exists() and newest & parse_test(TESTS / t).overrides.keys() for t in tests):
            thin[rid] = "rests only on tests with an override for a newest compiler"
    return thin


def head_classes() -> dict[str, str] | None:
    """{rule id: class} at HEAD, or None outside a git work tree."""
    import subprocess
    try:
        names = subprocess.run(["git", "ls-tree", "-r", "--name-only", "HEAD", "specification"],
                               cwd=ROOT, capture_output=True, text=True, check=True).stdout.split()
    except (OSError, subprocess.CalledProcessError):
        return None
    classes = {}
    for name in names:
        if not name.endswith(".md"):
            continue
        text = subprocess.run(["git", "show", f"HEAD:{name}"], cwd=ROOT,
                              capture_output=True, text=True, check=True).stdout
        directive = None
        for line in text.splitlines():
            if m := DIRECTIVE.match(line):
                directive = m.group(1)
            elif directive and (m := OPTION.match(line)) and m.group(1) == "kind":
                classes[directive] = m.group(2).strip()
            elif line.startswith(":::"):
                directive = None
    return classes


def check_thin_evidence(rules: dict, errors: list[str], warnings: list[str]) -> None:
    """Warn for each rule with thin evidence. Fail for such a rule when the
    working tree adds it or changes its class relative to HEAD."""
    thin = thin_evidence(rules)
    head = head_classes()
    if head is None:
        warnings.append("no git HEAD; the check of new and reclassified rules did not run")
    for rid, reason in sorted(thin.items(), key=lambda kv: (str(rules[kv[0]][0]), kv[0])):
        f, cls, _ = rules[rid]
        where = f.relative_to(ROOT)
        if head is not None and head.get(rid) != cls:
            change = "is new" if rid not in head else f"changed class from {head[rid]} to {cls}"
            errors.append(f"{where}: {rid} {change} and {reason}; add a test that every "
                          "newest compiler agrees on (documentation/specification-format.md)")
        else:
            warnings.append(f"{where}: {rid} {reason}")


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
        if test.default is None:
            errors.append(f"{test.name}: no base tidal-expect line (documentation/specification-format.md)")
        if test.allows_edg is not None:
            edg = test.overrides.get("edg")
            if test.allows_edg not in test.rules:
                errors.append(f"{test.name}: tidal-allows-edg names {test.allows_edg}, which tidal-rules does not cite")
            elif test.allows_edg in rules and rules[test.allows_edg][1] not in ALLOWS_EDG:
                errors.append(f"{test.name}: tidal-allows-edg names {test.allows_edg}, "
                              f"whose class {rules[test.allows_edg][1]} allows no second answer")
            if edg is None or test.default is None or (edg == "reject") == (test.default == "reject"):
                errors.append(f"{test.name}: tidal-allows-edg on a test whose EDG answer is the base answer")
        if test.errors and not {"reject", "link-fail", "run-fail"} & {test.default, *test.overrides.values()}:
            errors.append(f"{test.name}: tidal-error on a test that no compiler is expected to reject, fail to link, or fail to run")
        if test.default == "reject" and all(rid in rules for rid in test.rules) and not (
                {rules[rid][1] for rid in test.rules} & MAY_REJECT):
            errors.append(f"{test.name}: expects reject, but cites no rule of a class that allows it (decision 0034)")
        if "link-fail" in {test.default, *test.overrides.values()} and all(rid in rules for rid in test.rules) and (
                "no-diagnostic" not in {rules[rid][1] for rid in test.rules}):
            errors.append(f"{test.name}: expects a link failure, but cites no no-diagnostic rule (TR-4)")
        program = test.name.startswith(PROGRAMS)
        for rid in test.rules:
            if rid not in rules and rid.rsplit("-", 1)[0] not in prefixes:
                warnings.append(f"{test.name}: names {rid}; no {rid.rsplit('-', 1)[0]} rules are drafted yet")
            elif rid not in rules:
                errors.append(f"{test.name}: names unknown rule {rid}")
            elif program and rules[rid][1] == "withdrawn":
                errors.append(f"{test.name}: names {rid}, which is withdrawn")
            elif not program and rid not in listed.get(test.name, set()):
                errors.append(f"{test.name}: names {rid}, but {rid} doesn't list it")
        for rid in sorted(listed.get(test.name, set()) - set(test.rules)):
            errors.append(f"{test.name}: {rid} lists it, but its tidal-rules header doesn't cite {rid}")
    for t, rids in sorted(listed.items()):
        if t.startswith(PROGRAMS):
            errors.append(f"{t}: a program test is listed by {', '.join(sorted(rids))}; "
                          "rules list only the tests of their pages (decision 0054)")

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
              *(ROOT / "documentation").rglob("*.md")]:
        for ref in set(re.findall(r"\bQ-\d{4}\b", f.read_text())):
            if ref not in questions:
                errors.append(f"{f.relative_to(ROOT)}: refers to missing question {ref}")

    check_markers(errors)
    principles = set()
    for f in SPEC.rglob("*.md"):
        principles.update(re.findall(r"^:::\{principle\} (\S+)$", f.read_text(), re.M))
    for f in sorted([*SPEC.rglob("*.md"), *GUIDES.rglob("*.md")]):
        text = f.read_text()
        for role, ref in re.findall(r"\{(rule|principle)\}`([^`<]+?)`", text):
            if ref not in (rules if role == "rule" else principles):
                errors.append(f"{f.relative_to(ROOT)}: {{{role}}} refers to {ref}, which no page defines")
    import agreement
    import split_tests
    for f in sorted(SPEC.rglob("*.md")):
        if not agreement.current(f.read_text()):
            errors.append(f"{f.relative_to(ROOT)}: the agreement sentence is out of date; run tools/status_table.py")
        if not split_tests.current(f):
            errors.append(f"{f.relative_to(ROOT)}: the Split tests table is out of date; run tools/status_table.py")
    check_grammar(errors)
    check_records(errors, warnings)
    check_ledger(errors)
    check_evidence(errors, warnings)
    check_thin_evidence(rules, errors, warnings)

    for w in warnings:
        print("warning: " + w)
    for e in errors:
        print(e)
    print(f"{len(rules)} rules, {len(errors)} problems")
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main())
