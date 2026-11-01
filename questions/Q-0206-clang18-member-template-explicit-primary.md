# Q-0206: Are partial specializations of a member template ignored when its primary is explicitly specialized for one class?

**Status:** compiler defect
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

## Provisional answer

Option 1. Clang 22 agrees with GCC and EDG, so the Clang 18 result is a
defect that a later Clang fixes.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
