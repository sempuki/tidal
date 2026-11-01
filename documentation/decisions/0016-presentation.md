# 0016. Presentation: STE writing, FLS structure, and an HTML site

Date: 2026-10-30
Status: Accepted. Supersedes 0009. Amends 0005.

## Context

The draft sections contain too much prose. Rules mix several statements in
one paragraph, and some sentences use constructions that the project's prose
rules forbid. The PDF output suits print but does not suit search, linking,
or styling.

The Ferrocene Language Specification (FLS) for Rust solves the same
problems. Each FLS subchapter uses fixed segments: Syntax, Legality Rules,
Dynamic Semantics, Undefined Behavior, Implementation Requirements, and
informative Examples. Each paragraph has a stable ID and is short. Terms link
to a glossary. Sphinx builds it into a searchable HTML site.

## Decision

1. **Writing rules.** Specification text follows the ASD-STE100 writing
   rules as adapted in `documentation/style-guide.md`. C++ terms are technical
   names. A word list fixes the meaning of requirement words.
2. **Structure.** Each page uses the segments listed in the style guide,
   which follow the FLS segments. Rule classes map to segments:

   | Class | Segment |
   |---|---|
   | `required` (checked before run time), `reject` | Legality Rules |
   | `required` (at run time) | Dynamic Semantics |
   | `undefined` | Undefined Behavior |
   | `impl-defined`, `unspecified`, `no-diagnostic` | Implementation Requirements |
   | `recommended` | Recommended Practice |

3. **Examples.** Examples are informative code blocks in their own segment.
   Rules do not contain examples or explanations.
4. **Format.** Sources stay Markdown, written as MyST Markdown. Each rule is
   a `rule` directive with its ID, class, and tests. Terms use the Sphinx
   glossary. Grammar uses `productionlist`.
5. **Output.** Sphinx with the Furo theme builds an HTML site with search.
   CI builds it on every change and publishes it to GitHub Pages. The PDF
   build is removed.

## Consequences

- Every existing section is rewritten to the new structure and writing
  rules. Rule IDs and tests do not change.
- `tools/check_specification.py` and `tools/similarity.py` read rule directives.
- Contributors need Python and the pinned packages in
  `tools/requirements-site.txt` to build the site.
- Rule IDs stay mnemonic (`IF-3`). The FLS uses random IDs. Tidal keeps
  mnemonic IDs because tests and questions cite them.
