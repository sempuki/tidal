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

Used on 2026-11-04 by three source-reader gap reviews of the text phase 8 wrote. The classes and overloading review (`reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`) applies UNI-P4 to `bitfield-width-scoped-enumeration.cpp`, the declarations and statements review (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`) applies ALN-P1 to `operand-scoped-enumeration.cpp`, and the expressions, foundations, and templates review (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`) applies NEW-P1 to `array-scoped-enumeration-size.cpp`. Each model rejects the scoped enumerator (option 1), and each review agrees.

Kept open on 2026-11-04 under decision 0055, because UNI-P4 changed in the same commit: a width of class type now needs a literal class with exactly one `constexpr` conversion. The question waits for the next use of UNI-P4. In each form only one newest family differs.

Used on 2026-11-04 by the principle-use batch PU1 (`reports/clean-room/PU1/`), under decision 0056. A clean-room reader applied the principles, without the rule text, to the programs of `bitfield-width-scoped-enumeration.cpp`, `operand-scoped-enumeration.cpp`, and `array-scoped-enumeration-size.cpp`, and each answer gives the outcome that the tests expect (a rejection). The batch key lists UNI-P4 as the principle that the question waits on. Only the answer for `bitfield-width-scoped-enumeration.cpp` names UNI-P4. The answer for `operand-scoped-enumeration.cpp` names ALN-P1, and the answer for `array-scoped-enumeration-size.cpp` names NEW-P1. The batch score counts the use as not supporting the answer.

Kept open on 2026-11-04 under decision 0056, because the score of batch PU1 does not count the use as support: the answers for two of the three forms name ALN-P1 and NEW-P1, and the batch key lists only UNI-P4. Each of the three answers reaches the answer. In each form only one newest family differs.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. The `alignas` operand and the bit-field width must be integral constant expressions, which allow only integral and unscoped enumeration types. The size in an array `new` converts implicitly to the size type, and a scoped enumeration has no implicit conversion to an integer. EDG's and GCC's acceptances are defects.

### Gap review, 2026-11-04 (classes and overloading)

The source-reader gap review of the text phase 8 wrote on the classes and overloading chapters (`reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`) applies UNI-P4 to the width test and agrees with option 1. The width must be an integral constant expression, and such an expression takes only integral and unscoped enumeration types. A scoped enumerator converts to an integer only with a cast.

### Gap review, 2026-11-04 (declarations and statements)

The source-reader gap review of the text phase 8 wrote on the declarations and statements chapters (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`) applies ALN-P1 to the `alignas` test and agrees with option 1. The review adds that the bit-field width is the same kind of operand, and that the array size of a `new` expression converts implicitly to the size type, so a scoped enumerator is rejected in all three places.

### Gap review, 2026-11-04 (expressions, foundations, and templates)

The source-reader gap review of the text phase 8 wrote on the expressions, foundations, exceptions, and templates chapters (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`) applies NEW-P1 to the array size test and agrees with option 1 for that place. The other two places are on pages that the review did not cover.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
