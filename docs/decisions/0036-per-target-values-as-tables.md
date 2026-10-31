# 0036. Per-target values as tables

Date: 2026-10-31
Status: Accepted.

## Context

Roadmap item E adds a page of fundamental types. Many of its facts are
values that each target chooses: sizes, alignments, ranges, the
signedness of `char`. The matrix has two targets, x86-64 Linux (LP64)
and the same with `-m32` (ILP32). Written as sentences, such facts form
long word runs that repeat across types, and the similarity check
counts runs of words. The style guide limits a rule to three sentences
and lists for conditions, and it has no form for a value table.

## Decision

- An `impl-defined` rule can hold a table, one row per type and one
  column per matrix target. The rule's sentences say what the
  implementation chooses. The table gives the matrix targets' choices.
- Each table cell is backed by a test that checks the value on its
  target, for example with `static_assert` on `sizeof`.
- A list of type names that follows from a pattern is written as a rule
  that generates it, such as the specifier combinations for an integer
  type.
- `docs/style-guide.md` describes the form.

## Consequences

- Readers find a target's value without reading prose.
- A new target in the matrix adds a column to each table.
