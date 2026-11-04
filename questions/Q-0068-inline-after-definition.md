# Q-0068: Can a function be declared inline after its definition?

**Status:** settled
**Affects:** INL-2, `tests/declarations/inline/inline-after-definition.cpp`

## Context

A program defines a function without `inline`, then declares it again
with `inline`. Clang and EDG reject the second declaration. GCC accepts
it.

## Options

1. A function must not be declared `inline` after its definition.
2. A later `inline` declaration is valid.

## Answer

No; a function must not be declared `inline` after its definition (option 1). Clang and EDG agree.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) supports this answer with no doubt, and INL-P2 gives the same answer: `inline` is a promise about definitions, so the keyword must come no later than the definition. GCC 16 still differs. Tidal records GCC 16's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for GCC 16.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. The program is not valid when a definition is reachable at the first `inline` declaration, and a diagnostic is required.
