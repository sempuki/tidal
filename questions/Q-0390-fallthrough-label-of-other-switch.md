# Q-0390: Can a `fallthrough` statement lead to a label of an enclosing `switch`?

**Status:** provisional
**Affects:** ATTR-2, ATTR-P1, `tests/declarations/attributes/fallthrough-into-outer-switch.cpp`
**Source:** reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md, the ATTR-P1 section

## Context

A `fallthrough` statement is the last statement of an inner `switch`.
The next statement that can run after it has a `case` label of the
outer `switch`.

```cpp
switch (a) {
case 0:
  switch (b) {
  case 1:
    r = 1;
    [[fallthrough]];
  }
case 2:
  r += 2;
}
```

| Compiler | Result |
|---|---|
| Clang 22 | accepts, with no warning |
| GCC 16 | rejects when it compiles the code: "attribute 'fallthrough' not preceding a case label or default label". With `-fsyntax-only`, GCC accepts with no warning. |
| EDG 7.0 | rejects: "fallthrough statement must precede switch case label" |

A `fallthrough` in the outer `switch`, after the inner one, is valid
on every compiler (`fallthrough-same-switch-after-inner.cpp`).

## Evidence

The test in **Affects**. It expects a rejection and records
Clang's acceptance as an override.

## Options

1. The statement is not valid. The next statement must have a label of
   the same `switch` (GCC, EDG, and the review's reading).
2. The statement is valid. Any `case` or `default` label after it is
   enough (Clang).

## Provisional answer

The statement is not valid (option 1). The gap review reads the C++23
text this way, and GCC 16 and EDG 7.0 follow it. ATTR-P1 now gives the place of
`fallthrough` as a statement before a label of the same `switch`.
Tidal treats a misplaced standard attribute as not valid, as for
Q-0021.

Kept open on 2026-11-04 under decision 0054, because ATTR-P1 changed in this commit (decision 0055). Clang 22 is the one newest family that differs.

## Opinions

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on the declarations and statements chapters (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`) supports option 1. In its reading, a `fallthrough` statement is valid only where the next statement to run has a `case` or `default` label of the same `switch`. ATTR-P1 and ATTR-2 did not say "of the same `switch`", and the review called the case rare.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
