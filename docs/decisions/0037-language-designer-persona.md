# 0037. A language-designer persona

Date: 2026-10-31
Status: Accepted. Extends 0029 and 0030.

## Context

The maintainer asked for a reader who wants to add a feature to C++ and
uses Tidal as the base for the proposal. Such a reader tests breadth and
depth together. A proposal has to say how a new feature meets every
part of the language it touches: lookup, overloads, templates,
constant evaluation, object lifetime, and the rules a change would
break. The Java programmer and the front end test what a user and an
implementer need. Neither tests what a designer needs.

The maintainer described the persona as a WG21 participant who has read
many WG21 papers. AGENTS.md lists WG21 papers as protected material, so
an agent that draws on them is a source reader. A source reader writes
no files and gives only summaries without code or proposed wording. It
would also fill gaps in Tidal from memory, and those gaps are what the
exercise is meant to find.

## Decision

- The persona is a language designer who knows the craft of a proposal
  and takes all of its C++ knowledge from Tidal. It knows what a
  proposal must settle: motivation, design, interactions with existing
  rules, what existing programs it breaks, implementation cost, and the
  text a specification would change. It does not consult or recall WG21
  papers or the ISO text.
- Its features are its own inventions. If a feature looks like one
  that someone has proposed elsewhere, the reader works only from
  Tidal and its own design.
- Its first exercise is a roadmap: a portfolio of feature proposals that
  spans the breadth and depth of the language, and for each, the Tidal
  rules it would cite and the places where Tidal could not support the
  analysis. The report goes to `reports/clean-room/roadmap/designer.md`.
- A new exercise kind, Propose (X), follows: the reader writes the
  design and impact sections of one proposal from the portfolio,
  citing Tidal rule IDs, and logs each gap as a finding.
- Q-0116 records the alternative of a source-reader variant.

## Consequences

- The roadmap gains a fifth view.
- `docs/exercises.md` lists the persona and the Propose kind.
