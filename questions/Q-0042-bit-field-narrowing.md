# Q-0042: Is converting a bit-field whose values all fit the target a narrowing conversion?

**Status:** settled
**Affects:** LIST-2, `tests/declarations/initialization/list/narrowing-bit-field-fits.cpp`

## Context

`char c{s.bits};`, where `bits` is an `unsigned` bit-field of width 4, converts a value that always fits in `char`. Clang 22 and GCC 16 accept the initialization in braces. Clang 18, GCC 14, and EDG 7.0 reject it as narrowing.

## Options

1. Not narrowing when every value of the bit-field's width fits the target. The newer Clang and GCC agree; the older versions predate the change.
2. Narrowing, judged by the bit-field's declared type.

## Answer

Not narrowing when every value of the bit-field's width fits the target (option 1). The newest versions of two implementations accept the initialization, and both changed in the same direction.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on overloading, references, and list initialization (`reports/review/2026-11-03-gap-review-questions-overloading.md`) supports this answer with no doubt, and LIST-P1 gives the same answer: a value in braces is rejected only when it can lose information, and every value of the bit-field fits. EDG 7.0 still differs. Tidal records EDG 7.0's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for EDG 7.0.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on overloading, references, and list initialization (`reports/review/2026-11-03-gap-review-questions-overloading.md`) agrees with option 1. A bit-field narrower than its declared type is not narrowing when the target holds every value of an integer type of that width and the declared type's signedness. For an enumeration the width is compared with the underlying type's width. Clang 22 and GCC 16 follow this reading. Clang 18, GCC 14, and EDG follow the older rule.

