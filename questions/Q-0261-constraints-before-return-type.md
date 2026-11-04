# Q-0261: Are the constraints of a function template checked before its return type is formed?

**Status:** settled
**Affects:** CNS-11, `tests/templates/constraints/before-function-type.cpp`

## Context

The return type instantiates a class whose static assertion fails.
The constraint is not satisfied, and another function takes the call.

| Compiler | Result |
|---|---|
| Clang 18 | rejects: the static assertion fails |
| Clang 22, host | accepts |
| GCC 14, 16, host | accepts |
| EDG 7.0 | accepts |

## Evidence

The test above. Every compiler accepts the same case with the class in a parameter type (`before-parameters.cpp`).

Every compiler forms the default template arguments before the check.

## Options

1. The constraints come first, and a failure stops the substitution
   (Clang 22, GCC, EDG).
2. The return type comes first (Clang 18).

## Answer

Yes; every current compiler agrees, Clang 18 differs (option 1). Every current implementation agrees.

Settled on 2026-11-01 under decision 0051, because the newest version of each baseline family (Clang 22, GCC 16, EDG 7.0) agrees with this answer. Clang 18 still rejects the program, and the test keeps its override for Clang 18.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-11-01

The source-reader gap review of question cluster 1 (`reports/review/2026-11-01-gap-review-question-cluster-1.md`) supports the answer. Deduction checks constraints after it substitutes into the template parameter list and before it substitutes into the function type, so a failed constraint ends deduction before the return type is formed.
