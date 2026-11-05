# Q-0314: Can a constant of class type that converts to an integer give the operand of `alignas` or the width of a bit-field?

**Status:** settled
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

## Answer

Yes (ALN-8, BF-1), as Clang and EDG do; GCC rejects (option 1). Two of three compilers accept the operand, and the array
`new` size converts a class on every compiler. The tests expect an
acceptance and record GCC's result with an override.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the pages of ALN-8 and BF-1 says how a class operand gives an integer value. GCC 16 is the only newest family that differs.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). No principle on the two pages said how a class operand gives an integer value. ALN-P1 and UNI-P4 now say that the operand or the width can have a class type whose conversion function gives an integral or unscoped enumeration type, which rules out option 2. The existing tests check both places. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses ALN-P1 and UNI-P4.

Used on 2026-11-04 by two source-reader gap reviews of the text phase 8 wrote. The classes and overloading review (`reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`) applies UNI-P4 to `bitfield-width-class-converts.cpp`, and the declarations and statements review (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`) applies ALN-P1 to `operand-class-converts.cpp`. Each model accepts the operand by its conversion function (option 1), and each review agrees.

Kept open on 2026-11-04 under decision 0055, because UNI-P4 and BF-1 changed in the same commit. They now name the conditions the review found missing: a literal class, a `constexpr` conversion function, and one candidate conversion. The question waits for the next use of UNI-P4. GCC 16 is the only newest family that differs.

Used on 2026-11-04 by the principle-use batch PU1 (`reports/clean-room/PU1/`), under decision 0056. A clean-room reader applied the principles, without the rule text, to the programs of `operand-class-converts.cpp`, `bitfield-width-class-not-literal.cpp`, `bitfield-width-class-two-conversions.cpp`, `bitfield-width-class-not-constexpr.cpp`, `operand-class-conversion-ambiguous.cpp`, `bitfield-width-class-converts.cpp`, `operand-class-conversion-not-constexpr.cpp`, and `array-class-size.cpp`. Each answer gives the outcome that the tests expect. Five answers name UNI-P4. The answers for `operand-class-converts.cpp` and `operand-class-conversion-not-constexpr.cpp` name ALN-P1 with CE-P4 or CE-P1, and the answer for `array-class-size.cpp` names NEW-P1. For `operand-class-conversion-ambiguous.cpp` the reader borrowed a condition of UNI-P4, and for `operand-class-conversion-not-constexpr.cpp` the reader used CE-P1, because ALN-P1 lacks the conditions that UNI-P4 states (finding PU1-4). The use does not support the answer.

Kept open on 2026-11-04 under decision 0056, because the use in batch PU1 does not support the answer: three of the eight answers do not name UNI-P4, and finding PU1-4 shows that ALN-P1 does not give the `alignas` forms on its own. GCC 16 is the only newest family that differs.

Kept open on 2026-11-04 under decision 0055, because ALN-P1 and ALN-8 changed in this commit after finding PU1-4. ALN-P1 now asks of an operand of class type what UNI-P4 asks of a width: a literal class with exactly one `constexpr` conversion function to an integer type. A new test, `tests/declarations/alignment/operand-class-not-literal.cpp`, checks that every compiler rejects an operand whose class is not a literal type. The question waits for the next use of ALN-P1, UNI-P4, or NEW-P1. GCC 16 is the only newest family that differs.

Used on 2026-11-04 by the principle-use batch PU2 (`reports/clean-room/PU2/`), under decision 0056. A clean-room reader applied ALN-P1, UNI-P4, and NEW-P1, without the rule text, to the programs of `operand-class-converts.cpp`, `operand-class-conversion-ambiguous.cpp`, `operand-class-conversion-not-constexpr.cpp`, `operand-class-not-literal.cpp`, `bitfield-width-class-converts.cpp`, `bitfield-width-class-two-conversions.cpp`, `bitfield-width-class-not-constexpr.cpp`, `bitfield-width-class-not-literal.cpp`, and `array-class-size.cpp`. Each answer names the principle of its place and gives the outcome that the tests expect. The reader marked the reason for `bitfield-width-class-not-literal.cpp` and `operand-class-not-literal.cpp` doubtful, and not the outcome (finding PU2-4). The reader needed the glossary entry for a literal type, and CE-P1 seemed to allow a temporary of any type. The use supports the answer.

Settled on 2026-11-04 under decision 0054. The use in batch PU2 supports the answer, the gap reviews of 2026-11-04 support it with no doubt, and ALN-P1 and UNI-P4 give the same answer by the test of the style guide (section 3). Finding PU2-4 does not keep the question open. Its doubt concerns why a class that is not a literal type fails, a condition beside the options of the question, and both of the reader's paths give the rejection. The answers for `operand-class-converts.cpp` and `bitfield-width-class-converts.cpp`, which decide between the options, carry no doubt. GCC 16 still differs, and the tests keep their overrides.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. Where an integral constant expression is required, an expression of literal class type converts contextually to an integer or unscoped enumeration type, and the chosen conversion function must be `constexpr`. The `alignas` operand and the bit-field width are such places. GCC's rejection is a defect.

### Gap review, 2026-11-04 (classes and overloading)

The source-reader gap review of the text phase 8 wrote on the classes and overloading chapters (`reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`) applies UNI-P4 to the width test and agrees with option 1. A value of literal class type converts contextually to an integral or unscoped enumeration type where an integral constant expression is needed. The review finds that UNI-P4 leaves out three conditions: the class must be a literal type, the chosen conversion function must be `constexpr`, and two candidate conversion functions make the width ambiguous. The test meets all three, so the answer stands.

### Gap review, 2026-11-04 (declarations and statements)

The source-reader gap review of the text phase 8 wrote on the declarations and statements chapters (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`) applies ALN-P1 to the `alignas` test and agrees with option 1. The class converts to an integer by its conversion function. The review notes the same two conditions for ALN-P1 that it notes for the width: the chosen conversion function must be `constexpr`, and the conversion must be unambiguous.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
