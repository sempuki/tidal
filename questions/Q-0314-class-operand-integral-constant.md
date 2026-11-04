# Q-0314: Can a constant of class type that converts to an integer give the operand of `alignas` or the width of a bit-field?

**Status:** provisional
**Affects:** ALN-8, BF-1, `tests/declarations/alignment/operand-class-converts.cpp`, `tests/classes/unions/bitfield-width-class-converts.cpp`

## Context

A class has a `constexpr` conversion function to `int`, and a
`constexpr` variable of the class is the operand of `alignas` or the
width of a bit-field.

| Place | Clang 22 | GCC 16 | EDG 7.0 |
|---|---|---|---|
| `alignas(eight)` | accepts, alignment 8 | rejects | accepts |
| a bit-field width `three` | accepts | rejects | accepts |

GCC says that the operand or the width has a non-integral type. Every
compiler accepts a size of class type in an array `new` (NEW-2).

## Evidence

The two tests above, and
`tests/expressions/new-delete/array-class-size.cpp`.

## Options

1. The class converts by its conversion function, and the result gives
   the value. Clang and EDG do this.
2. Only a value of integral or unscoped enumeration type is valid. GCC
   does this.

## Provisional answer

Yes (ALN-8, BF-1), as Clang and EDG do; GCC rejects (option 1). Two of three compilers accept the operand, and the array
`new` size converts a class on every compiler. The tests expect an
acceptance and record GCC's result with an override.

Kept open on 2026-11-03 under decision 0054, because no principle on the pages of ALN-8 and BF-1 says how a class operand gives an integer value. GCC 16 is the only newest family that differs.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. Where an integral constant expression is required, an expression of literal class type converts contextually to an integer or unscoped enumeration type, and the chosen conversion function must be `constexpr`. The `alignas` operand and the bit-field width are such places. GCC's rejection is a defect.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
