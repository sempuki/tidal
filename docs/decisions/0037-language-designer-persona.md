# 0037. A language-designer persona

Date: 2026-10-31
Status: Accepted. Extends 0029 and 0030.

## Context

The clean-room readers test what a user needs (the Java programmer) and
what an implementer needs (the front end). No reader tests what a
language designer needs. A proposal for a new feature has to say how the
feature meets every part of the language it touches, such as:

- lookup and overloads,
- templates,
- constant evaluation,
- object lifetime,
- the existing programs whose meaning it changes.

So a designer tests breadth and depth together.

AGENTS.md lists WG21 papers as protected material. A designer who draws
on them is a source reader, who writes no files and gives only summaries
without code or proposed wording. Such a reader would also fill gaps in
Tidal from memory, and those gaps are what the exercise is meant to
find.

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
