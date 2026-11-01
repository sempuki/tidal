# Q-0276: When an earlier element fixes the length of a trailing pack of bases, does each later element still deduce?

**Status:** provisional
**Affects:** CTAD-27, `tests/templates/class-template-deduction/aggregate-pack-length-conflict.cpp`

## Context

`template <class... T> struct Bases : List<T...>, T... {};` and then
`Bases b{List<X, Y, Z>{}, {}, ToY{}};`, where `ToY` converts to `Y`.
The first element fixes `T` as `X, Y, Z`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects: no viable guide |
| EDG 7.0 | rejects: no constructor matches |
| GCC 14, 16, host | accepts |

With `{}` as the last element, every compiler deduces `Bases<X, Y, Z>`.

## Evidence

The test above and `aggregate-pack-length-fixed.cpp`. Item 28 of the
gap review `reports/review/2026-10-31-gap-review-packs-and-ctad.md` predicts a deduction failure on conflict.

## Options

1. The last element deduces `ToY` for the third element of `T`, which
   conflicts with `Y`, so deduction fails (Clang, EDG).
2. The last element is matched with the fixed type and converts (GCC).

## Provisional answer

Option 1. Two of three implementations agree, and the gap review
predicts the same result.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
