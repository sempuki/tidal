"""Sphinx extension for Tidal specification pages.

Provides:
- the `principle` directive, which renders one principle with the rules that
  are its evidence;
- the `rule` directive, which renders one rule with its ID, class, and tests;
- the `rule` role, which links to a rule by ID: {rule}`IF-3`;
- the `question` role, which links to a question file: {question}`Q-0001`.

See docs/style-guide.md for how rules are written.
"""

import re
from pathlib import Path

from docutils import nodes
from docutils.parsers.rst import directives
from sphinx.util.docutils import SphinxDirective

KINDS = {
    "required", "reject", "no-diagnostic", "impl-defined", "unspecified",
    "undefined", "recommended", "withdrawn",
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
    option_spec = {"rules": directives.unchanged}

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


def in_progress_notice(app, docname, source):
    """Put a notice at the top of a spec page that has no rules yet."""
    text = source[0]
    if docname.endswith("index") or docname == "glossary" or ":::{rule}" in text:
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
    app.connect("source-read", in_progress_notice)
    return {"version": "1", "parallel_read_safe": True}
