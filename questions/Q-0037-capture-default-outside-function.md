# Q-0037: May a lambda outside a function have a capture default?

**Status:** provisional
**Affects:** LAM-6, `tests/expressions/lambdas/capture-default-at-namespace.cpp`

## Context

`auto g = [=] { return 1; };` at namespace scope has a capture default but nothing to capture. Clang 22 and 18 and GCC 16 and 14 reject the lambda ("non-local lambda expression cannot have a capture-default"). EDG 7.0 accepts it.

## Options

1. Rejected: a capture default needs an enclosing function.
2. Accepted, because the lambda has nothing to capture.

## Provisional answer

No; rejected (option 1). Clang and GCC, in both versions, reject the lambda.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page gives the answer. LAM-P2 and LAM-P3 say what a closure captures, and they do not say where a capture default can appear. EDG 7.0 accepts the lambda.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). LAM-P3 failed step 3, because no principle said where a capture default can appear. Item PQ widened LAM-P3: a capture default asks for captures from an enclosing function or, in a default member initializer, through `this`, so a lambda at namespace scope cannot have one. The new test `tests/expressions/lambdas/capture-default-in-member-initializer.cpp` checks the default member initializer case, which no test checked. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses LAM-P3.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. A capture default or a simple capture is allowed only when the lambda's innermost enclosing scope is a block, or when the lambda is in a default member initializer and the innermost scope is that class. A lambda at namespace scope is neither, whether or not anything would be captured. EDG's acceptance is a defect. LAM-6 already states the exception for a default member initializer.
