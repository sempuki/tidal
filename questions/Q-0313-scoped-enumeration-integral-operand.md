# Q-0313: Can an enumerator of a scoped enumeration give the operand of `alignas`, the width of a bit-field, or the size in an array `new`?

**Status:** provisional
**Affects:** ALN-8, BF-2, NEW-2, `tests/declarations/alignment/operand-scoped-enumeration.cpp`, `tests/classes/unions/bitfield-width-scoped-enumeration.cpp`, `tests/expressions/new-delete/array-scoped-enumeration-size.cpp`

## Context

A scoped enumeration has no implicit conversion to an integer type.
Three places take an integer value and split on such an enumerator:

| Place | Clang 22 | GCC 16 | EDG 7.0 |
|---|---|---|---|
| `alignas(Width::eight)` | rejects | rejects | accepts |
| a bit-field width `Bits::three` | rejects | rejects | accepts |
| `new int[Count::two]` | rejects | accepts | rejects |

Every compiler accepts an enumerator of an unscoped enumeration in all
three places.

## Evidence

The three tests above. Clang says that the operand must have integral
or unscoped enumeration type. GCC says that the `alignas` operand and
the width have a non-integral type. EDG says that the `new` size must
have integral or unscoped enumeration type.

## Options

1. A scoped enumeration is not valid in any of the three places. Each
   place takes a value of integral or unscoped enumeration type, as
   Clang does everywhere.
2. Each place converts a scoped enumerator to its integer value.

## Provisional answer

No (ALN-8, BF-2, NEW-2); EDG accepts it in `alignas` and a width, GCC in `new` (option 1). Two of three compilers reject the enumerator in each place,
and option 1 gives the three places one rule. The tests expect a
rejection and record GCC's and EDG's results with overrides.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the pages of ALN-8, BF-2, and NEW-2 gives the answer. ENUM-P2, on the enumerations page, says that a scoped enumeration converts to an integer only with a cast. In each form only one newest family differs.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). No principle on the three pages gave the answer, and ENUM-P2 is on another page that the three pages did not cite. The matrix shows one model, in which each place takes a value only by an implicit conversion, and a scoped enumeration has none. ALN-P1 (the `alignas` operand), UNI-P4 (the width), and NEW-P1 (the array size) now state the model and cite ENUM-P2. The existing tests check each place. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses ALN-P1, UNI-P4, and NEW-P1.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. The `alignas` operand and the bit-field width must be integral constant expressions, which allow only integral and unscoped enumeration types. The size in an array `new` converts implicitly to the size type, and a scoped enumeration has no implicit conversion to an integer. EDG's and GCC's acceptances are defects.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
