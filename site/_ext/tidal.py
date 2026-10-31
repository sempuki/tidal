"""Sphinx extension for Tidal specification pages.

Provides:
- the `principle` directive, which renders one principle with the rules that
  are its evidence;
- the `rule` directive, which renders one rule with its ID, class, and tests;
- the `rule` role, which links to a rule by ID: {rule}`IF-3`;
- the `question` role, which links to a question file: {question}`Q-0001`;
- the `tidal-undefined-index` directive, which lists the `undefined` rules and
  the compilers whose constant evaluation rejects each one.

See docs/style-guide.md for how rules are written.
"""

import fnmatch
import os
import re
import sys
from pathlib import Path

from docutils import nodes
from docutils.parsers.rst import directives
from sphinx.util.docutils import SphinxDirective

KINDS = {
    "required", "reject", "no-diagnostic", "impl-defined", "unspecified",
    "undefined", "recommended", "conditionally-supported", "withdrawn",
}


class RuleDirective(SphinxDirective):
    required_arguments = 1
    has_content = True
    option_spec = {
        "kind": lambda v: directives.choice(v, sorted(KINDS)),
        "tests": directives.unchanged,
    }

    def run(self):
        rid = self.arguments[0]
        kind = self.options.get("kind", "required")
        target_id = "rule-" + rid.lower()
        repo = self.config.tidal_repository

        box = nodes.container(classes=["tidal-rule", f"tidal-kind-{kind}"], ids=[target_id])
        head = nodes.paragraph(classes=["tidal-rule-head"])
        head += nodes.reference("", rid, refid=target_id, classes=["tidal-rule-id"])
        head += nodes.inline("", kind, classes=["tidal-rule-kind"])
        box += head

        body = nodes.container(classes=["tidal-rule-body"])
        self.state.nested_parse(self.content, self.content_offset, body)
        box += body

        tests = self.options.get("tests", "").split()
        if tests:
            foot = nodes.paragraph(classes=["tidal-rule-tests"])
            foot += nodes.inline("", "Tests: ")
            for i, t in enumerate(tests):
                if i:
                    foot += nodes.inline("", ", ")
                foot += nodes.reference("", Path(t).name,
                                        refuri=f"{repo}/blob/main/tests/{t}")
            box += foot

        std = self.env.get_domain("std")
        std.note_object("tidalrule", rid, target_id, location=box)
        return [box]


class PrincipleDirective(SphinxDirective):
    """One principle: prose that the listed rules follow from."""
    required_arguments = 1
    has_content = True
    option_spec = {"rules": directives.unchanged, "exceptions": directives.unchanged}

    def run(self):
        pid = self.arguments[0]
        target_id = "principle-" + pid.lower()
        box = nodes.container(classes=["tidal-principle"], ids=[target_id])
        head = nodes.paragraph(classes=["tidal-principle-head"])
        head += nodes.reference("", pid, refid=target_id, classes=["tidal-principle-id"])
        box += head
        body = nodes.container(classes=["tidal-principle-body"])
        self.state.nested_parse(self.content, self.content_offset, body)
        box += body
        rules = self.options.get("rules", "").split()
        if rules:
            foot = nodes.paragraph(classes=["tidal-principle-rules"])
            foot += nodes.inline("", "Rules: ")
            for i, rid in enumerate(rules):
                if i:
                    foot += nodes.inline("", ", ")
                foot += nodes.reference("", rid, refid="rule-" + rid.lower(),
                                        classes=["tidal-rule-ref"])
            box += foot
        exceptions = self.options.get("exceptions", "").split()
        if exceptions:
            foot = nodes.paragraph(classes=["tidal-principle-rules"])
            foot += nodes.inline("", "Exceptions: ")
            for i, rid in enumerate(exceptions):
                if i:
                    foot += nodes.inline("", ", ")
                foot += nodes.reference("", rid, refid="rule-" + rid.lower(),
                                        classes=["tidal-rule-ref"])
            box += foot
        std = self.env.get_domain("std")
        std.note_object("tidalprinciple", pid, target_id, location=box)
        return [box]


def page_facts(path: Path):
    """Return (title, summary, rule count) from a spec page's source."""
    text = path.read_text()
    lines = text.splitlines()
    title = next((l[2:].strip() for l in lines if l.startswith("# ")), path.stem)
    summary, started = [], False
    for l in lines[1:]:
        if l.startswith("#") or l.startswith(":::") or l.startswith("```") or l.startswith("- "):
            if started:
                break
            continue
        if l.strip():
            started = True
            summary.append(l.strip())
        elif started:
            break
    rules = sum(1 for l in lines if l.startswith(":::{rule}"))
    sentences = re.split(r"(?<=[.:])\s+", " ".join(summary))
    keep = [s for s in sentences
            if not s.startswith("Rules are prefixed") and not s.endswith(":")]
    return title, " ".join(keep), rules


class PagesDirective(SphinxDirective):
    """List the pages of a section with their summaries and rule counts.

    Content lines are page names relative to the current document, as in a
    toctree. The facts are read from the page sources, so they can't drift.
    """
    has_content = True

    def run(self):
        here = Path(self.env.doc2path(self.env.docname)).parent
        table = nodes.table(classes=["tidal-pages"])
        group = nodes.tgroup(cols=3)
        table += group
        for width in (30, 55, 15):
            group += nodes.colspec(colwidth=width)
        head = nodes.thead()
        group += head
        row = nodes.row()
        for label in ("Page", "Summary", "Rules"):
            row += nodes.entry("", nodes.paragraph(text=label))
        head += row
        body = nodes.tbody()
        group += body
        for name in (l.strip() for l in self.content if l.strip()):
            src = here / f"{name}.md"
            if not src.exists():
                src = here / name / "index.md"
            title, summary, rules = page_facts(src)
            self.env.note_dependency(str(src))
            link_target = f"{name}.html" if (here / f"{name}.md").exists() else f"{name}/index.html"
            row = nodes.row()
            link = nodes.paragraph()
            link += nodes.reference("", title, refuri=link_target)
            row += nodes.entry("", link)
            row += nodes.entry("", *self.parse_text_to_nodes(summary))
            count = str(rules) if rules else "in progress"
            row += nodes.entry("", nodes.paragraph(text=count))
            body += row
        return [table]


RULE_LINE = re.compile(r"^:::\{rule\}\s+([A-Z]+-\d+)\s*$")
PRINCIPLE_LINE = re.compile(r"^:::\{principle\}\s+([A-Z]+-P\d+)\s*$")


def page_outline(path: Path):
    """Return (title, [(principle id, first sentence, rules, exceptions)], {rule: (kind, ntests)})."""
    lines = path.read_text().splitlines()
    title = next((l[2:].strip() for l in lines if l.startswith("# ")), path.stem)
    principles, rules = [], {}
    cur = None
    for line in lines:
        if m := PRINCIPLE_LINE.match(line):
            cur = {"id": m.group(1), "rules": [], "exceptions": [], "text": []}
            principles.append(cur)
        elif m := RULE_LINE.match(line):
            cur = None
            rid = m.group(1)
            rules[rid] = ["required", 0]
            current_rule = rid
        elif line.startswith(":kind:") and rules:
            rules[current_rule][0] = line.split(":", 2)[2].strip()
        elif line.startswith(":tests:") and rules:
            rules[current_rule][1] = len(line.split(":", 2)[2].split())
        elif cur is not None:
            if line.startswith(":rules:"):
                cur["rules"] = line.split(":", 2)[2].split()
            elif line.startswith(":exceptions:"):
                cur["exceptions"] = line.split(":", 2)[2].split()
            elif line == ":::":
                cur = None
            elif line.strip() and not cur["text"]:
                cur["text"].append(line.strip())
    for pr in principles:
        first = " ".join(pr["text"])
        pr["gist"] = re.split(r"(?<=[.])\s", first)[0] if first else ""
    return title, principles, rules


class RuleIndexDirective(SphinxDirective):
    """List every page with its principles, each principle's rules, and test counts."""
    has_content = False

    def run(self):
        root = Path(self.env.srcdir)
        order = ["foundations", "statements", "expressions", "declarations", "classes", "exceptions"]
        out = []
        for chapter in order:
            index = root / chapter / "index.md"
            if not index.exists():
                continue
            sec = nodes.section(ids=[f"rule-index-{chapter}"])
            sec += nodes.title(text=chapter.capitalize())
            for page in self._pages(index):
                self.env.note_dependency(str(page))
                title, principles, rules = page_outline(page)
                rel = page.relative_to(root).with_suffix("").as_posix()
                psec = nodes.section(ids=[f"rule-index-{rel.replace('/', '-')}"])
                head = nodes.title()
                head += nodes.reference("", title, refuri=f"{rel}.html")
                psec += head
                cited = set()
                for pr in principles:
                    item = nodes.paragraph()
                    item += nodes.reference("", pr["id"], refuri=f"{rel}.html#principle-{pr['id'].lower()}",
                                            classes=["tidal-rule-ref"])
                    item += nodes.inline("", " " + pr["gist"] + " ")
                    for key, label in (("rules", ""), ("exceptions", "except ")):
                        for rid in pr[key]:
                            cited.add(rid)
                            kind, n = rules.get(rid, ("?", 0))
                            item += nodes.inline("", " ")
                            item += nodes.reference("", f"{label}{rid}", refuri=f"{rel}.html#rule-{rid.lower()}",
                                                    classes=["tidal-rule-ref", f"tidal-kind-{kind}"])
                            item += nodes.inline("", f"\u2009({n})", classes=["tidal-index-count"])
                    psec += item
                loose = [r for r in rules if r not in cited]
                if loose:
                    item = nodes.paragraph()
                    item += nodes.inline("", "Rules not under a principle: " + ", ".join(loose))
                    psec += item
                sec += psec
            out.append(sec)
        return out

    def _pages(self, index: Path):
        text = index.read_text()
        m = re.search(r":::\{tidal-pages\}\n(.*?)\n:::", text, re.S)
        if not m:
            return
        for name in m.group(1).split():
            target = index.parent / f"{name}.md"
            if name.endswith("index"):
                yield from self._pages(index.parent / f"{name}.md")
            elif target.exists():
                yield target


FAMILIES = (("clang", "Clang"), ("gcc", "GCC"), ("edg", "EDG"))


def undefined_rules(root: Path):
    """Yield (rule id, page path, first sentence) for each `undefined` rule, in page order."""
    for page in sorted(root.rglob("*.md")):
        lines = page.read_text().splitlines()
        for i, line in enumerate(lines):
            m = RULE_LINE.match(line)
            if not m:
                continue
            j = i + 1
            kind = None
            while j < len(lines) and lines[j].startswith(":"):
                if lines[j].startswith(":kind:"):
                    kind = lines[j].split(":", 2)[2].strip()
                j += 1
            if kind != "undefined":
                continue
            text = []
            for l in lines[j:]:
                if l == ":::" or (text and not l.strip()):
                    break
                if l.strip():
                    text.append(l.strip())
            first = re.split(r"(?<=[.:])\s", " ".join(text))[0]
            yield m.group(1), page, first


class UndefinedIndexDirective(SphinxDirective):
    """List every `undefined` rule and the compilers whose constant evaluation rejects it.

    The marks come from the test expectations, which the matrix checks: a
    compiler rejects a rule if a test of the rule expects `reject` from it.
    Tests with -Werror check a warning, not constant evaluation, so they
    don't count.
    """
    has_content = False

    def run(self):
        root = Path(self.env.srcdir)
        tools = Path(self.env.config.tidal_root) / "tools"
        if str(tools) not in sys.path:
            sys.path.insert(0, str(tools))
        import matrix
        import run_tests

        compilers = [c for c in matrix.load() if c.role == "baseline"]
        tests = {}
        for path in run_tests.test_files([matrix.TESTS]):
            self.env.note_dependency(str(path))
            t = run_tests.parse_test(path)
            if "-Werror" in t.flags:
                continue
            for rid in t.rules:
                tests.setdefault(rid, []).append(t)
        self.env.note_dependency(str(matrix.CONFIG))

        here = Path(self.env.docname).parent.as_posix()
        table = nodes.table(classes=["tidal-undefined-index"])
        group = nodes.tgroup(cols=2 + len(FAMILIES))
        table += group
        for width in [12, 58] + [10] * len(FAMILIES):
            group += nodes.colspec(colwidth=width)
        head = nodes.thead()
        group += head
        row = nodes.row()
        for label in ["Rule", "Operation"] + [name for _, name in FAMILIES]:
            row += nodes.entry("", nodes.paragraph(text=label))
        head += row
        body = nodes.tbody()
        group += body
        for rid, page, first in undefined_rules(root):
            self.env.note_dependency(str(page))
            target = page.relative_to(root).with_suffix(".html").as_posix()
            uri = f"{os.path.relpath(target, here)}#rule-{rid.lower()}"
            row = nodes.row()
            link = nodes.paragraph()
            link += nodes.reference("", rid, refuri=uri, classes=["tidal-rule-ref"])
            row += nodes.entry("", link)
            row += nodes.entry("", *self.parse_text_to_nodes(first))
            for family, _ in FAMILIES:
                row += nodes.entry("", nodes.paragraph(text=self._mark(
                    tests.get(rid, []), [c for c in compilers if c.family == family])))
            body += row
        return [table]

    @staticmethod
    def _mark(tests, compilers):
        """How the compilers of one family treat a rule's rejection tests.

        Each compiler rejects every case, some cases, or none. A family whose
        compilers differ lists them by id.
        """
        cases = [t for t in tests if t.default == "reject" or "reject" in t.overrides.values()]
        if not cases:
            return "no test"

        def status(c):
            ran = [t for t in cases
                   if not any(fnmatch.fnmatch(f, p) for f in t.flags for p in c.unsupported_flags)]
            n = sum(t.expect(c) == "reject" for t in ran)
            return "rejects" if n == len(ran) else "accepts" if n == 0 else "some cases"

        by_status = {}
        for c in compilers:
            by_status.setdefault(status(c), []).append(c.id)
        if len(by_status) == 1:
            return next(iter(by_status))
        return "; ".join(f"{', '.join(ids)}: {st}" for st, ids in by_status.items())


def in_progress_notice(app, docname, source):
    """Put a notice at the top of a spec page that has no rules yet."""
    text = source[0]
    if (docname.endswith("index") or docname in ("glossary", "rules") or docname.startswith("guides/")
            or ":::{rule}" in text):
        return
    lines = text.split("\n", 1)
    notice = (":::{note}\nThis page is in progress. Its tests exist, and its rules "
              "are not written yet.\n:::\n")
    source[0] = lines[0] + "\n\n" + notice + (lines[1] if len(lines) > 1 else "")


def question_role(name, rawtext, text, lineno, inliner, options=None, content=None):
    env = inliner.document.settings.env
    root = Path(env.config.tidal_root) / "questions"
    matches = sorted(root.glob(f"{text}-*.md"))
    if not matches:
        msg = inliner.reporter.error(f"unknown question {text}", line=lineno)
        return [inliner.problematic(rawtext, rawtext, msg)], [msg]
    url = f"{env.config.tidal_repository}/blob/main/questions/{matches[0].name}"
    return [nodes.reference(rawtext, text, refuri=url, classes=["tidal-question"])], []


def setup(app):
    app.add_config_value("tidal_repository", "", "env")
    app.add_config_value("tidal_root", "", "env")
    app.add_crossref_type("tidalrule", "rule")
    app.add_crossref_type("tidalprinciple", "principle")
    app.add_directive("principle", PrincipleDirective)
    app.add_directive("rule", RuleDirective, override=True)
    app.add_role("question", question_role)
    app.add_directive("tidal-pages", PagesDirective)
    app.add_directive("tidal-rule-index", RuleIndexDirective)
    app.add_directive("tidal-undefined-index", UndefinedIndexDirective)
    app.connect("source-read", in_progress_notice)
    return {"version": "1", "parallel_read_safe": True}
