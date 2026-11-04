# Q-0207: Does an explicit specialization of one member partial specialization keep the others?

**Status:** settled
**Affects:** TSP-21

## Context

A class template `Outer` has a member template `Inner` with partial
specializations for `U*` and `U&`. The program gives
`template <> template <class U> struct Outer<short>::Inner<U*>`.

| Compiler | `Outer<short>::Inner<int*>` | `Inner<int&>` | `Inner<int>` |
|---|---|---|---|
| Clang 18, 22, host | the explicit one | the `U&` one | the primary |
| GCC 14, 16, host | the `U*` partial specialization from the class template | the `U&` one | the primary |
| EDG 7.0 | the explicit one | the `U&` one | the primary |

## Evidence

`tests/templates/specialization/member-template-explicit-partial.cpp`,
with an override for GCC.

## Options

1. The explicit specialization replaces the `U*` partial
   specialization for `Outer<short>`, and the others still apply (Clang,
   EDG).
2. The explicit specialization is not selected (GCC).

## Answer

Yes, and it is selected; Clang and EDG agree, GCC ignores it (option 1). Clang and EDG agree, and the explicit specialization would
otherwise have no use.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) supports this answer with no doubt, and TSP-P2 gives the same answer: an explicit specialization for the argument list is used before any partial specialization. GCC 16 still differs. Tidal records GCC 16's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for GCC 16.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) agrees with option 1. When one partial specialization of a member template is explicitly specialized for a single class specialization, the primary member template and the other partial specializations stay in play for that class specialization, and the explicit one replaces the partial one it specializes. GCC's choice is a defect.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
