# Q-0094: Is a class-specific operator delete without a specifier non-throwing?

**Status:** settled
**Affects:** EXS-4, `tests/declarations/exception-specifications/class-delete-operator-noexcept.cpp`

## Context

A class declares a static `operator delete(void*, size)` with no
`noexcept` specifier. Clang and EDG treat a direct call of it as
non-throwing. GCC treats the call as potentially throwing.

GCC 14 and 16 give the same result, and so do Clang 18 and 22. The host
GCC 16.2 and Clang 22.1 agree with them. No newer version settles the
split.

## Options

1. A deallocation function is non-throwing without the keyword.
2. The function follows the general rule and is potentially throwing.

## Answer

Yes; a deallocation function is non-throwing without the keyword (option 1). Clang and EDG agree.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) supports this answer with no doubt, and EXS-P3 gives the same answer: its table makes a deallocation function non-throwing when the declaration says nothing. GCC 16 still differs. Tidal records GCC 16's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for GCC 16.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. Every deallocation function with no explicit exception specifier is non-throwing. The rule has no exception for a class member or for a form with a size parameter, so GCC's result is a defect.
