# Q-0028: Is converting a floating value above the target's finite range implementation-defined or undefined?

**Status:** provisional
**Affects:** CONV-11, `tests/foundations/conversions/floating-narrowing-out-of-range.cpp`
**Waits for:** a review (doubt)

## Context

`constexpr float f = 1e300;` converts a `double` that is larger than the
largest finite `float`. Clang 22 and 18 and GCC 16 and 14 accept it during
constant evaluation and give positive infinity. EDG 7.0 rejects it
("floating-point value does not fit in required floating-point type").

On IEEE 754 targets, infinity is a value of `float`, so 1e300 lies between
the largest finite `float` and infinity. A value between two values of the
target is a rounding case. A value outside every value of the target is
out of range.

## Options

1. `impl-defined`: on targets where the floating type has infinities, the
   result is one of the two neighboring values. Clang and GCC conform; EDG
   is stricter than necessary.
2. `undefined`: any value above the largest finite value is out of range.
   EDG conforms, and Clang and GCC accept an extension.

## Provisional answer

Implementation-defined (option 1). This option matches the run-time behavior of every compiler on
IEEE targets, and two of three implementations during constant evaluation.

Kept open on 2026-11-03 under decision 0054, because the gap review does not support the answer with no doubt (it reads with doubt, weakly, unclearly, or for only part of the question).

Kept open on 2026-11-11 under decision 0054 (decision 0066, item DQ). The second review supports the answer with a narrower doubt (DQ-2). EDG 7.0 alone rejects, and the rules of the tests let either result stand. The doubt is whether an infinity counts as a neighboring value of the target. Q-0001 rests on the opposite reading of the same point (DQ-1), so the two answers cannot both follow from one reading.

## Opinions

### Gap review summary, 2026-10-30

**Position:** option 2.

A reviewer who read the reference
(`reports/review/2026-10-30-gap-review.md`) says the reference treats a
value outside the target's range as undefined, and allows an
implementation's choice only between two neighboring values. The reviewer
notes that counting infinity as a neighbor is a reading the reference
doesn't clearly support. The writer keeps option 1 provisional until a
contributor settles which reading applies.

### Second review of the doubt-held questions, 2026-11-11

The source-reader second review of the questions held only by a doubt (`reports/review/2026-11-11-second-review-doubt-held-questions.md`) reads the question: supports with a doubt that remains (DQ-2). A conversion between floating types has three cases: an exact value, a value between two neighboring values of the target, and every other value, which is undefined. The answer holds only if an infinity counts as a neighboring value. The doubt is the doubt of DQ-1, and the reference is silent on it in both places. The review asks that one reading serve both Q-0001 and this question.
