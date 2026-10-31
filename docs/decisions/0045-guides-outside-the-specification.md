# 0045. Guides outside the specification

Date: 2026-10-31
Status: Accepted. Changes the place of the guides from the roadmap's
phase 2.

## Context

The Diátaxis framework sorts documentation into four kinds by the
reader's need:

| | For study | For work |
|---|---|---|
| Practical steps | tutorial | how-to guide |
| Knowledge | explanation | reference |

A specification is reference material. Tidal's guides teach tasks, so
they are tutorials and how-to guides. The guides sat in `spec/guides/`
as a chapter of the specification, which mixed two kinds in one
document.

Writing a guide from the specification alone still tests the
specification. A guide that needs a fact that no rule states shows a
gap in the rules.

## Decision

- The specification is reference material only. No page of `spec/` is
  a tutorial, a how-to guide, or an explanation.
- The guides move to `guides/` at the top of the repository.
- The site's front page, `site/index.md`, has a Diátaxis table that
  links the tutorial, the how-to guides, and the specification.
  `tools/build_site.py` links `site/index.md`, `spec/`, and `guides/`
  into one Sphinx source tree. `spec/index.md` becomes the page
  `specification`.
- Guide writing is a clean-room exercise of kind Guide (G) in
  `docs/exercises.md`. Its findings go to the rule pages, and a guide
  never states a fact that no rule states.
- `tools/check_spec.py` checks question references, markers, and rule
  and principle references in `guides/`. `tools/similarity.py` checks
  `spec/` and `guides/` by default.

## Consequences

- The explanation quadrant has no pages yet. Each spec page's
  principles stay in the reference, because they state the model that
  the page's rules follow.
- A link from a spec page to a guide is not allowed. A guide links to
  the rules.
