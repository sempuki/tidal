# Q-0134: Does a bit-field narrower than `int` but declared with a wider type have a promotion to `int` in a conversion sequence?

**Status:** provisional
**Affects:** ICS-2, `tests/foundations/conversion-sequences/bit-field-promotion.cpp`

## Context

A bit-field of width 3 is declared `long long`. CONV-1 says that such a
bit-field promotes to `int` in arithmetic. The question is whether the
conversion to `int` in a conversion sequence then has promotion rank.

Two functions take `int` and `long`, and the argument is the bit-field.

| Compiler | Result |
|---|---|
| Clang 18, Clang 22 | chooses `int` (a promotion beats a conversion) |
| GCC 14, GCC 16 | ambiguous |
| EDG 7.0 | ambiguous |

The arithmetic grid shows the same split for every pair with `int`.

## Evidence

`tests/foundations/conversion-sequences/bit-field-promotion.cpp`, and
the row "long long : 3 bit-field" of the arithmetic grid on the page.

## Options

1. The conversion to `int` is an integer promotion, as in CONV-1, and
   has promotion rank (Clang).
2. The conversion follows the declared type `long long`, so every
   integer target has conversion rank (GCC, EDG).

## Provisional answer

Option 1. ICS-2 gives the integer promotions of CONV-1 promotion rank,
and CONV-1 decides a bit-field's promotion by its width. One rule for
arithmetic and for sequences is simpler.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
