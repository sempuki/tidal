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

Yes; Clang 22 and GCC 16 agree, Clang 18, GCC 14, and EDG differ (option 1). The newest Clang and GCC agree, every compiler already drops
the candidate when the constraint fails, and a type constraint on a
type parameter takes part in the comparison in the same way.

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 choose the constrained template, and EDG 7.0 finds the call ambiguous, so two families disagree. Clang 18 and GCC 14 agree with EDG. CNS-P1 supports the provisional answer. Step 1a collects the type constraints of constant parameters into the associated constraints, so the comparison of CNS-P4 sees them (CNS-23). EDG's behavior is on the cluster 5 list of EDG differences that waits on the maintainer (`documentation/status.md`).

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
