# 0046. The site publishes the specification alone

Date: 2026-10-31
Status: Accepted. Supersedes the site part of 0045.

## Context

Decision 0045 moved the guides out of `specification/` and gave the site a front
page with a Diátaxis table that linked the guides beside the
specification. The site then presented itself as a specification and
offered guides on its first page.

## Decision

- The site publishes the specification and nothing else. The Sphinx
  source is `specification/`, and `specification/index.md` is the front page.
- The guides stay in `guides/` in the repository, unpublished, until a
  later decision gives them a place.
- The rest of 0045 stands. The specification is reference material
  only, a specification page does not link to a guide, guide writing is a
  clean-room exercise whose findings go to the rule pages, and the
  checks cover `guides/`.

## Consequences

- `tools/build_site.py` builds `specification/` directly. `site/index.md` is
  removed.
