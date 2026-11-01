# Q-0289: Is a nested requirement that evaluates a parameter of the requires expression an error?

**Status:** provisional
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

## Provisional answer

Option 1. Every current implementation agrees.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
