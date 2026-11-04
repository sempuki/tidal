# Q-0014: May an implementation perform dynamic initialization before the program starts?

**Status:** settled
**Affects:** SINIT-4, `tests/declarations/initialization/static/maybe-static.cpp`

## Context

`int b = three();`, where `three` is an `inline` function that returns 3
but isn't `constexpr`, is a dynamic initialization. An initializer earlier
in the same unit that reads `b` sees 0 if `b` is initialized at run time in
order, and 3 if it was initialized before the program started.

## Evidence

Clang 22 and GCC 16 at `-O0` and `-O2` give 0. GCC 16 with
`-fimplicit-constexpr` gives 3. The test accepts both.

## Options

1. `unspecified`: an implementation may do the initialization early when
   the result is the same. GCC's option is one such implementation choice.
2. `required` in-order initialization, which forbids GCC's option.

## Answer

Yes, unspecified (option 1). The default behavior of every compiler fits option 1, and
option 1 doesn't rule out an implementation choice that already exists.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) supports this answer with no doubt, and SINIT-P2 gives the same answer: an implementation can do a dynamic initialization early, when the value is the same. No newest family differs from the answer.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. An implementation can turn a dynamic initialization into a static one under conditions on the other variables and on the final value. When an earlier initializer in the same unit reads such a variable, which of the two values the reader sees is unspecified. Both values in the test are allowed.
