# Q-0289: Is a nested requirement that evaluates a parameter of the requires expression an error?

**Status:** settled
**Affects:** CPT-35, CNS-9, `tests/templates/concepts/nested-requirement-parameter-evaluated.cpp`

## Context

A concept has `requires (T t) { requires t.value == 0; }`, where
`value` is a non-static data member. The parameter `t` has no value.

| Compiler | Result |
|---|---|
| Clang 22, host | rejects: non-constant expression |
| Clang 18 | accepts: the concept-id is `false` |
| GCC 14, 16, host | rejects: t is not a constant expression |
| EDG 7.0 | rejects: atomic constraint not constant |

## Evidence

The test above. Every compiler accepts a nested requirement that uses
the parameter only in an operand of `sizeof`
(`nested-requirement-parameter-unevaluated.cpp`). The gap review of
concepts and constraints (item 22) expects the rejection.

## Options

1. An error, as for any atomic constraint that is not a constant
   expression (CNS-9).
2. The requirement does not hold (Clang 18).

## Answer

Yes; every current compiler agrees, Clang 18 differs (option 1). Every current implementation agrees.

Settled on 2026-11-01 under decision 0051, because the newest version of each baseline family (Clang 22, GCC 16, EDG 7.0) agrees with this answer. Clang 18 still accepts the program and gives `false`, and the test keeps its override for Clang 18.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-11-01

The source-reader gap review of question cluster 1 (`reports/review/2026-11-01-gap-review-question-cluster-1.md`) supports the answer. The parameters of a requires expression have no storage, so reading a member of one is not a constant expression, and an atomic constraint must be one. The program is ill-formed.
