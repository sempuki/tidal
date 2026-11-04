# Q-0107: Can constinit apply to a structured binding declaration?

**Status:** provisional
**Affects:** SB-11, `tests/declarations/structured-bindings/constinit-specifier.cpp`

## Context

A structured binding declaration at namespace scope has `constinit`. Clang and GCC 16 reject the declaration. EDG 7.0 and GCC 14 accept it.

## Options

1. Not valid; only static, thread_local, const, and volatile can occur.

2. Valid.

## Provisional answer

No; only static, thread_local, const, and volatile can occur (option 1). The newer GCC agrees with Clang.

Kept open on 2026-11-03 under decision 0054, because no principle on the page gives the answer. SB-P3 allows a few specifiers and does not say which. EDG 7.0 is the only newest family that differs.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). SB-P3 failed step 3, because the model allowed "a few specifiers" without naming them. SB-P3 now allows no specifiers other than `static`, `thread_local`, `const`, and `volatile`, so `constinit` is not valid. The review reads with no doubt for C++23, and EDG 7.0 is the one newest family that differs. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses SB-P3.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. A structured binding declaration allows only `static`, `thread_local`, the placeholder, and the cv-qualifiers as declaration specifiers, so `constinit` is not allowed. A later change for the next standard allows `constexpr` on a structured binding. The review has not checked whether that change also allows `constinit`.
