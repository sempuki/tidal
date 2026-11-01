# Q-0244: Does a parenthesized list in a `new` expression form the aggregate deduction candidate?

**Status:** provisional
**Affects:** CTAD-9, `tests/templates/class-template-deduction/aggregate-new-parentheses.cpp`

## Context

`template <class T> struct Agg { T v; };` and then `new Agg(1)`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts, `Agg<int>*` |
| EDG 7.0 | accepts, `Agg<int>*` |
| GCC 14, 16, host | rejects: class template argument deduction failed |

GCC forms the aggregate candidate for `Agg a(1);`, for `Agg(1)`, and
for `new Agg{1}`.

## Evidence

The test above and the grid `forms.json` (row "new", column
"aggregate").

## Options

1. The candidate is formed for a parenthesized list in every place
   (Clang, EDG).
2. The candidate is formed for a parenthesized list only outside a
   `new` expression (GCC).

## Provisional answer

Option 1. Two of three implementations agree, and GCC itself forms the
candidate for the same list in a variable declaration and a functional
cast.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
