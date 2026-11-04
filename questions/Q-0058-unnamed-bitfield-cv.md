# Q-0058: Can an unnamed bit-field have a cv-qualifier?

**Status:** provisional
**Affects:** BF-2, `tests/classes/unions/bitfield-unnamed-cv.cpp`

## Context

An unnamed bit-field only reserves bits. A program cannot name it, so a
`const` or `volatile` on it has no effect. Clang and EDG reject the
qualifier. GCC accepts it.

## Options

1. A program must not put a cv-qualifier on an unnamed bit-field.
2. The qualifier is valid and has no effect.

## Provisional answer

No; a program must not put a cv-qualifier on an unnamed bit-field (option 1). Clang and EDG agree, and the qualifier cannot change the
meaning of the program.

Kept open on 2026-11-03 under decision 0054, because no principle on the page states the case. UNI-P4 says that an unnamed bit-field only reserves bits that no operation reads or writes, which gives a qualifier no effect, and does not say whether the qualifier is allowed. GCC 16 is the one newest family that differs.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). UNI-P4 failed step 3, because it gave the qualifier no effect and did not say whether the qualifier is allowed, so both options fit. UNI-P4 now says that an unnamed bit-field takes no cv-qualifier and that a qualifier on it is rejected. The existing test checks the case. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses UNI-P4.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) agrees with option 1. A cv-qualified type on an unnamed bit-field is not allowed, and GCC's acceptance is a defect.
