# Q-0043: Does parenthesized aggregate initialization allow brace elision?

**Status:** settled
**Affects:** LIST-9, `tests/declarations/initialization/list/paren-no-brace-elision.cpp`

## Context

`Out o(1, 2, 3);`, where `Out` holds an inner aggregate of two `int` members and then an `int`, needs brace elision to fill the inner member. Clang 22 and 18 and GCC 16 and 14 reject the initialization. EDG 7.0 accepts it and fills the inner aggregate from the first two values.

## Options

1. No brace elision in parentheses. EDG's acceptance is a possible defect.
2. Brace elision applies in parentheses, as EDG does.

## Answer

No; parentheses allow no brace elision (option 1). Clang and GCC, in both versions, reject the initialization.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on overloading, references, and list initialization (`reports/review/2026-11-03-gap-review-questions-overloading.md`) supports this answer with no doubt, and LIST-P2 gives the same answer: parentheses give one value per part and no elision. EDG 7.0 still differs. Tidal records EDG 7.0's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for EDG 7.0.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on overloading, references, and list initialization (`reports/review/2026-11-03-gap-review-questions-overloading.md`) agrees with option 1. Each element of a parenthesized aggregate initialization takes exactly one value, and braces are never left out there. An inner aggregate cannot take one integer, so the program is not valid.

