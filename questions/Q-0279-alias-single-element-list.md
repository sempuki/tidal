# Q-0279: Does one element of the class type skip the initializer-list phase through an alias template?

**Status:** provisional
**Affects:** CTAD-18, `tests/templates/class-template-deduction/list-single-specialization-alias.cpp`

## Context

`template <class T> struct List { List(std::initializer_list<T>); };`,
`template <class T> using Alias = List<T>;`, a `List<int> one`, and then
`Alias copy{one};`. Without the alias, every compiler deduces
`List<int>` (CTAD-18).

| Compiler | Result |
|---|---|
| Clang 22, host | `List<int>` |
| Clang 18 | rejects: no deduction through an alias (Q-0240) |
| GCC 14, 16, host | `List<int>` |
| EDG 7.0 | `List<List<int>>` |

## Evidence

The test above. Item 36 of the gap review `reports/review/2026-10-31-gap-review-packs-and-ctad.md` says that the skip of
the initializer-list phase applies also through an alias.

## Options

1. The alias gives the same result as the class template, `List<int>`
   (Clang 22, GCC).
2. The initializer-list guide is tried first through an alias (EDG).

## Provisional answer

Yes; Clang 22 and GCC agree, EDG differs (option 1). Two of three implementations agree, and the gap review
predicts the same result.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
