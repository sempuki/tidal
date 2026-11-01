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

Option 1. Two of three implementations agree, and EDG rejects the case
with equal constraints.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
