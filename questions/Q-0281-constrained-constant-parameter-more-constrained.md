# Q-0281: Is a function template with a constrained constant parameter more constrained than the same template with `auto`?

**Status:** provisional
**Affects:** CNS-1, CNS-23, CPT-21, `tests/templates/constraints/constant-placeholder-more-constrained.cpp`

## Context

Two function templates differ only in their constant parameter. One
declares `Wide auto N`, the other `auto N`. A call with the argument
`5` satisfies both.

| Compiler | Result |
|---|---|
| Clang 22, host | chooses the constrained template |
| GCC 16, host | chooses the constrained template |
| Clang 18 | rejects: the call is ambiguous |
| GCC 14 | rejects: the call is ambiguous |
| EDG 7.0 | rejects: the call is ambiguous |

## Evidence

The test above. With the argument `'a'`, every compiler drops the
constrained template and calls the other one
(`placeholder-constant-parameter-fallback.cpp`). The gap review of
concepts and constraints (item 2) describes the constraint of the
placeholder as a constraint of the template.

## Options

1. The type constraint of the placeholder is part of the associated
   constraints, so the constrained template is more constrained
   (Clang 22, GCC 16).
2. The type constraint is checked only for the argument's type, and
   takes no part in the comparison (Clang 18, GCC 14, EDG).

## Provisional answer

Option 1. The newest Clang and GCC agree, every compiler already drops
the candidate when the constraint fails, and a type constraint on a
type parameter takes part in the comparison in the same way.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
