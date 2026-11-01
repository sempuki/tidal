# Q-0207: Does an explicit specialization of one member partial specialization keep the others?

**Status:** provisional
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

## Provisional answer

Option 1. Clang and EDG agree, and the explicit specialization would
otherwise have no use.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
