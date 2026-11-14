# 0067. Chapter 0: the statement of provenance

Date: 2026-11-14
Status: Accepted. Extends 0045 and 0046, which stand.

## Context

Tidal's text must not come from the ISO standard, and the project's
rules (`AGENTS.md`, decision 0020, `documentation/clean-room.md`) say
how. Those rules live in the repository. A reader of the site found no
statement of them, so the site gave no answer to a doubt about where
the text comes from. The maintainer asked for a clear statement that is
easy to find, on the site, as chapter 0.

## Decision

- `specification/introduction.md` is chapter 0, "How Tidal is made". It
  is the statement of provenance of the specification: the front matter
  that says who writes the text, what each writer may read, and how the
  text improves without the text of the ISO standard. It states no
  language rules and holds no tests.
- The first paragraph of the front page states the project's respect
  for copyright and for the work of the contributors to the ISO C++
  standard, and links chapter 0. The front page lists chapter 0 in a
  toctree of its own before the numbered chapters 1 to 8.
- Chapter 0 opens with the same statement of respect.
- Chapter 0 names each specification and each program it mentions in a
  table at its start: the ISO standard (ISO/IEC 14882:2024), the Tidal
  specification (the pages whose source is `specification/`), the
  baseline compilers, and the Tidal front ends (`frontend/`). Its text
  uses those names only, so that no sentence leaves open which
  specification it means.
- Chapter 0 holds a diagram of the workflow, drawn in SVG with the
  theme's colors.

## Consequences

- Decisions 0045 and 0046 stand. The site publishes the specification
  alone, chapter 0 is part of the specification as its front matter,
  no page is a tutorial or a how-to guide, and no page links to a guide.
- Chapter 0 describes the process in `AGENTS.md`,
  `documentation/clean-room.md`, and `documentation/pipeline.md`. A
  change to the roles or the pipeline changes chapter 0 in the same
  commit.
