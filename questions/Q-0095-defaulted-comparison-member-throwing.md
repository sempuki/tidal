# Q-0095: Is a defaulted == non-throwing if a member == is potentially throwing?

**Status:** settled
**Affects:** EXS-4, `tests/declarations/exception-specifications/defaulted-comparison-member-throwing.cpp`

## Context

A class defaults `operator==`. A member's class has an `==`
without `noexcept`. Clang and GCC treat the defaulted `==` as potentially
throwing. EDG 7.0 treats it as non-throwing.

GCC 14 and 16 give the same result, and so do Clang 18 and 22. The host
GCC 16.2 and Clang 22.1 agree with them. No newer version settles the
split.

## Options

1. The defaulted comparison is potentially throwing, as for defaulted
   special members.
2. The defaulted comparison is non-throwing.

## Answer

No; the defaulted comparison is potentially throwing, as for defaulted special members (option 1). Clang and GCC agree, and the rule matches
defaulted special members.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) supports this answer with no doubt, and EXS-P3 gives the same answer: a defaulted special member or comparison is non-throwing only when every part that the function touches is non-throwing. EDG 7.0 still differs. Tidal records EDG 7.0's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for EDG 7.0.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. A comparison operator defaulted on its first declaration with no exception specifier is potentially throwing exactly when an expression in its implicit definition is potentially throwing. A call of a member's `==` that has no non-throwing specifier is such an expression. EDG's result is a defect.
