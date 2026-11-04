# Q-0206: Are partial specializations of a member template ignored when its primary is explicitly specialized for one class?

**Status:** settled
**Affects:** TSP-21

## Context

A class template `Outer` has a member template `Inner` with a pointer
partial specialization. The program gives
`template <> template <class U> struct Outer<short>::Inner` and uses
`Outer<short>::Inner<int*>`.

| Compiler | Result |
|---|---|
| Clang 22, host | uses the explicit specialization |
| Clang 18 | uses the pointer partial specialization |
| GCC 14, 16, host | uses the explicit specialization |
| EDG 7.0 | uses the explicit specialization |

## Evidence

`tests/templates/specialization/member-template-explicit-primary.cpp`,
with an override for Clang 18.

## Options

1. The partial specializations are not used for `Outer<short>` (all but
   Clang 18).
2. The partial specializations still apply (Clang 18).

## Answer

Yes; Clang 18's choice is fixed in Clang 22 (option 1). Clang 22 agrees with GCC and EDG, so the Clang 18 result is a
defect that a later Clang fixes.

Settled on 2026-11-01 under decision 0051, because the newest version of each baseline family (Clang 22, GCC 16, EDG 7.0) agrees with this answer. Clang 18 still uses the pointer partial specialization, and the test keeps its override for Clang 18.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-11-01

The source-reader gap review of question cluster 1 (`reports/review/2026-11-01-gap-review-question-cluster-1.md`) supports the answer. An explicit specialization of the primary member template for one specialization of the outer class drops the member's partial specializations for that outer specialization.
