"""Sphinx extension for Tidal specification pages.

Provides:
- the `rule` directive, which renders one rule with its ID, class, and tests;
- the `rule` role, which links to a rule by ID: {rule}`IF-3`;
- the `question` role, which links to a question file: {question}`Q-0001`.

See docs/style-guide.md for how rules are written.
"""

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
    app.add_directive("rule", RuleDirective, override=True)
    app.add_role("question", question_role)
    return {"version": "1", "parallel_read_safe": True}
