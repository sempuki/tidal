# Q-0277: How does a string literal initialize an array member in the aggregate candidate?

**Status:** provisional
**Affects:** CTAD-9, CTAD-10, `tests/templates/class-template-deduction/aggregate-dependent-element.cpp`, `aggregate-string-fixed-array.cpp`

## Context

Two forms give different results.

`template <class T> struct Three { T v[3]; };` and then
`Three w{"ab"};`. The element type of the array depends on `T`, and
the bound does not.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts, `Three<const char*>` |
| GCC 14, 16, host | accepts, `Three<const char*>` |
| EDG 7.0 | accepts, `Three<const char*>` |

`template <class T> struct Named { char v[3]; T n; };` and then
`Named b{"ab", 1};`. The array does not depend on `T`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts, `Named<int>` |
| GCC 14, 16, host | accepts, `Named<int>` |
| EDG 7.0 | rejects: no constructor matches |

## Evidence

The tests above. Item 26 of the gap review `reports/review/2026-10-31-gap-review-packs-and-ctad.md` says that braces are not
elided for an array member with a dependent element type when the
element of the list is a string literal. Under that reading,
`Three w{"ab"}` gives `Three<char>`.

## Options

1. The braces of an array with a fixed bound are elided, also for a
   string literal. A string literal initializes a whole array of
   `char` that does not depend on a parameter. Every compiler agrees on
   the first form, and Clang and GCC on the second.
2. A string literal for an array with a dependent element type keeps
   its braces, as the gap review reads.

## Provisional answer

Braces elided for a fixed bound; Clang and GCC agree, EDG differs on a fixed char array (option 1). Every implementation elides the braces in the first form,
and two of three accept the second.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
