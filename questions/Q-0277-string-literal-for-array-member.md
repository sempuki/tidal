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

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 accept the string literal for the fixed `char` array and deduce `Named<int>`, and EDG 7.0 finds no guide, so two families disagree on the second form. Every compiler agrees on the first form. CTAD-P2 supports the provisional answer. The aggregate candidate gives an array member that a string literal initializes an lvalue reference to the `const` array type (CTAD-9). For `Named b{"ab", 1}`, the candidate then has the parameters `const char(&)[3]` and `T`, and `T` is `int`. EDG's behavior is on the cluster 5 list of EDG differences that waits on the maintainer (`documentation/status.md`).

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
