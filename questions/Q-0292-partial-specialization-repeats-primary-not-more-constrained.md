# Q-0292: Can a partial specialization repeat the primary template's parameters when it is not more constrained?

**Status:** provisional
**Affects:** CNS-36, TSP-11, `tests/templates/constraints/partial-specialization-not-more-constrained.cpp`

## Context

The primary template has the head `<A T>`, and the partial
specialization has the head `<B T>` and the argument list `<T>`.
Neither `A` nor `B` subsumes the other.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects: not more specialized than the primary template |
| GCC 14, 16, host | rejects: not more constrained than the primary template |
| EDG 7.0 | accepts |

## Evidence

The test above. Every compiler accepts the form when the partial
specialization is more constrained
(`partial-specialization-same-arguments.cpp`), and rejects it when both
have the same constraint (`partial-specialization-same-constraints.cpp`).

## Options

1. Not valid (Clang, GCC). For an argument that satisfies both
   constraints, neither template is more specialized, so the choice
   would be ambiguous.
2. Valid (EDG).

## Provisional answer

No; Clang and GCC agree, EDG differs (option 1). Two of three implementations agree, and EDG rejects the case
with equal constraints.

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 reject the partial specialization, and EDG 7.0 accepts it, so two families disagree. TSP-P3 supports the provisional answer. A partial specialization whose argument list is just the primary template's parameters must be more constrained than the primary template, or it covers no argument list that the primary template does not (CNS-36, TSP-11). EDG 7.0 rejects the same form with equal constraints. EDG's behavior is on the cluster 5 list of EDG differences that waits on the maintainer (`documentation/status.md`).

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
