# 0030. A catalog of spec-only exercises

Date: 2026-10-31
Status: Accepted. Extends 0029.

## Context

The first spec-only programmer (W0) found twelve problems in five
programs. One reader with one task list reaches only the pages its
tasks touch. Covering the whole specification needs many exercises,
and they need a scheme so that their coverage can be counted and
their findings compared.

## Decision

- Each exercise has a kind, a persona, and a set of target pages.
- The kinds are Write (a program from a prose task), Port (a program
  translated from the persona's language), Predict (the outcome of a
  Tidal test with its directives removed), and Diagnose (the rule
  behind a compiler or sanitizer message).
- Predict and Diagnose batches are generated from the test suite, one
  batch per page, and scored automatically against the tests'
  expectations. Write and Port tasks are written by hand.
- A persona is a background language whose habits are likely to
  mislead on the target pages: Java, C#, OCaml, Haskell, Rust, Go, Ada,
  and Swift.
- `documentation/exercises.md` lists every planned exercise: fourteen Write
  suites that together reach every specification page, eight Port suites, and a
  Predict and a Diagnose batch for each page. The list also gives the
  order and status of the exercises.
- Reports go under `reports/spec-only/<exercise-id>/` and cite rule
  IDs, so a coverage table can count the exercises and findings for
  each page.

## Consequences

- The Predict and Diagnose batches need a tool that removes the
  directives from a test and scores the answers.
- The pilot report stays at `reports/spec-only/java-programmer/` and
  counts as W0.
