# Q-0100: Is a character literal with value zero a null pointer constant?

**Status:** provisional
**Affects:** CONV-8, `tests/expressions/conversions/char-zero-not-null.cpp`

## Context

A pointer is initialized from a character literal whose value is zero. Clang and GCC reject the initialization. EDG 7.0 accepts it as a null pointer.

## Options

1. Only an integer literal with value zero is a null pointer constant.

2. A character literal with value zero is one too.

## Provisional answer

No; only an integer literal with value zero is a null pointer constant (option 1). Clang and GCC agree.

Kept open on 2026-11-03 under decision 0054, because no principle on the page gives the answer. CONV-P3 says that a null pointer comes only from `nullptr` or a literal zero, and it does not say whether a character literal with the value zero counts as a literal zero. EDG 7.0 is the only newest family that differs.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. A null pointer constant is an integer literal with the value zero or a prvalue of the type of `nullptr`. A character literal is a separate kind of literal and does not count. EDG follows an older rule, in which any integral constant expression with the value zero counted.
