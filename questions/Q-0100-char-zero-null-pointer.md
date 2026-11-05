# Q-0100: Is a character literal with value zero a null pointer constant?

**Status:** provisional
**Affects:** CONV-8, `tests/foundations/conversions/char-zero-not-null.cpp`

## Context

A pointer is initialized from a character literal whose value is zero. Clang and GCC reject the initialization. EDG 7.0 accepts it as a null pointer.

## Options

1. Only an integer literal with value zero is a null pointer constant.

2. A character literal with value zero is one too.

## Provisional answer

No; only an integer literal with value zero is a null pointer constant (option 1). Clang and GCC agree.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page gives the answer. CONV-P3 says that a null pointer comes only from `nullptr` or a literal zero, and it does not say whether a character literal with the value zero counts as a literal zero. EDG 7.0 is the only newest family that differs.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). CONV-P3 failed step 3, because "a literal zero" did not say whether a character literal counts. Item PQ widened CONV-P3: a null pointer comes only from `nullptr` or an integer literal with the value zero, and the principle names `'\0'`, `false`, and an integer variable as numbers that are not null pointers. CONV-8 and its tests are already among the evidence of CONV-P3. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses CONV-P3.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. A null pointer constant is an integer literal with the value zero or a prvalue of the type of `nullptr`. A character literal is a separate kind of literal and does not count. EDG follows an older rule, in which any integral constant expression with the value zero counted.
