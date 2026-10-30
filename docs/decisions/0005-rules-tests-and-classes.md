# 0005. Every rule has a class and at least one executable test

Date: 2026-10-30
Status: Accepted

## Context

LLM-drafted prose is cheap to produce and expensive to verify. Without expert
reviewers, the proof of concept needs a mechanical way to tell whether a rule
matches what implementations do.

## Decision

- The spec is a list of numbered rules, one requirement each.
- Each rule carries one of six classes: required, reject, no-diagnostic,
  impl-defined, unspecified, undefined.
- Each rule cites at least one test file. Tests run on Clang and GCC with
  `-std=c++23 -pedantic-errors`.
- A tool checks that every rule has a test and every test names real rules.

Format details are in `docs/spec-format.md`. The process is in
`docs/pipeline.md`.

## Consequences

- Tests are the part of Tidal that can be checked without reading the prose.
  A future reviewer can audit coverage by reading tests alone.
- `undefined`, `unspecified`, and `no-diagnostic` rules can only be supported
  indirectly (constant evaluation, sanitizers, variation). Their tests show
  evidence, not proof.
- The structure (rule IDs, classes) is Tidal's own and does not follow ISO
  numbering or stable names.
