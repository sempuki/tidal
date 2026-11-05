# Q-0134: Does a bit-field narrower than `int` but declared with a wider type have a promotion to `int` in a conversion sequence?

**Status:** provisional
**Affects:** ICS-2, `tests/overloading/conversion-sequences/bit-field-promotion.cpp`
**Waits for:** a review (doubt)

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

`tests/overloading/conversion-sequences/bit-field-promotion.cpp`, and
the row "long long : 3 bit-field" of the arithmetic grid in `tests/overloading/conversion-sequences/grids/tables.md`.

## Options

1. The conversion to `int` is an integer promotion, as in CONV-1, and
   has promotion rank (Clang).
2. The conversion follows the declared type `long long`, so every
   integer target has conversion rank (GCC, EDG).

## Provisional answer

Yes; Clang agrees, GCC and EDG differ (option 1). ICS-2 gives the integer promotions of CONV-1 promotion rank,
and CONV-1 decides a bit-field's promotion by its width. One rule for
arithmetic and for sequences is simpler.

Kept open on 2026-11-03 under decision 0054, because the review reads the answer with doubt.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on overloading, references, and list initialization (`reports/review/2026-11-03-gap-review-questions-overloading.md`) agrees with option 1, with doubt. A bit-field converts to `int` by an integral promotion when `int` holds every value of the bit-field, with no condition on the declared type, so the sequence has promotion rank. A note on conversion sequences says that a sequence ignores whether the argument is a bit-field. Read strictly, the note gives GCC's and EDG's result. The note is not binding, so the review leans to option 1.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
