# Q-0276: When an earlier element fixes the length of a trailing pack of bases, does each later element still deduce?

**Status:** settled
**Affects:** CTAD-27, `tests/templates/class-template-deduction/aggregate-pack-length-conflict.cpp`, `tests/templates/class-template-deduction/aggregate-pack-length-position.cpp`

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
With `Y{}` as the last element, every compiler deduces `Bases<X, Y, Z>`.
Every compiler rejects `Bases b{List<X, Y, Z>{}, Y{}};`, where `Y{}`
stands in the position of the base `X`. So each later element is
matched with the base in its position on every compiler. The compilers
differ only on an element that converts to its base.

## Evidence

The test above, `aggregate-pack-length-fixed.cpp`, and
`aggregate-pack-length-position.cpp`. Item 28 of the
gap review `reports/review/2026-10-31-gap-review-packs-and-ctad.md` predicts a deduction failure on conflict.

## Options

1. The last element deduces `ToY` for the third element of `T`, which
   conflicts with `Y`, so deduction fails (Clang, EDG).
2. The last element is matched with the fixed type and converts (GCC).

## Answer

Yes; Clang and EDG agree, GCC differs (option 1). Two of three implementations agree, and the gap review
predicts the same result.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) supports this answer with no doubt, and CTAD-P2 gives the same answer: the class's own parameters are deduced as a function template's parameters are, so the values from every pair must agree. GCC 16 still differs. Tidal records GCC 16's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for GCC 16.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) agrees with option 1. The aggregate candidate matches the trailing pack of bases to the remaining elements, and ordinary deduction runs on every pair. An element of a class type deduces that type for its position in the pack, and a type that differs from the one fixed earlier makes the deductions inconsistent. No step skips deduction for a pack whose length is known.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
