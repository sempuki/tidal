# Q-0100: Is a character literal with value zero a null pointer constant?

**Status:** settled
**Affects:** CONV-8, `tests/foundations/conversions/char-zero-not-null.cpp`

## Context

A pointer is initialized from a character literal whose value is zero. Clang and GCC reject the initialization. EDG 7.0 accepts it as a null pointer.

## Options

1. Only an integer literal with value zero is a null pointer constant.

2. A character literal with value zero is one too.

## Answer

No; only an integer literal with value zero is a null pointer constant (option 1). Clang and GCC agree.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page gives the answer. CONV-P3 says that a null pointer comes only from `nullptr` or a literal zero, and it does not say whether a character literal with the value zero counts as a literal zero. EDG 7.0 is the only newest family that differs.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). CONV-P3 failed step 3, because "a literal zero" did not say whether a character literal counts. Item PQ widened CONV-P3: a null pointer comes only from `nullptr` or an integer literal with the value zero, and the principle names `'\0'`, `false`, and an integer variable as numbers that are not null pointers. CONV-8 and its tests are already among the evidence of CONV-P3. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses CONV-P3.

Used on 2026-11-04 by the source-reader gap review of the text phase 8 wrote on expressions, foundations, and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`), Part B. The review applied CONV-P3 to the question's program, a pointer initialized from a character literal whose value is zero (`char-zero-not-null.cpp`), without the rule text: a character literal is not an integer literal, so it is no null pointer constant. That is the provisional answer, and the model rules out the other options.

Settled on 2026-11-04 under decision 0054. The review supports the answer with no doubt, and CONV-P3 gives the same answer under the test of the style guide (section 3). EDG 7.0 still differs, and the tests keep their overrides.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. A null pointer constant is an integer literal with the value zero or a prvalue of the type of `nullptr`. A character literal is a separate kind of literal and does not count. EDG follows an older rule, in which any integral constant expression with the value zero counted.

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on expressions, foundations, and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`) agrees with option 1. A null pointer constant is an integer literal with value zero or a prvalue of the type of `nullptr`. A character literal is not an integer literal.
