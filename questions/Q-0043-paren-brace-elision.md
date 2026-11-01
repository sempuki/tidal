# Q-0043: Does parenthesized aggregate initialization allow brace elision?

**Status:** provisional
**Affects:** LIST-9, `tests/declarations/initialization/list/paren-no-brace-elision.cpp`

## Context

`Out o(1, 2, 3);`, where `Out` holds an inner aggregate of two `int` members and then an `int`, needs brace elision to fill the inner member. Clang 22 and 18 and GCC 16 and 14 reject the initialization. EDG 7.0 accepts it and fills the inner aggregate from the first two values.

## Options

1. No brace elision in parentheses. EDG's acceptance is a possible defect.
2. Brace elision applies in parentheses, as EDG does.

## Provisional answer

No; parentheses allow no brace elision (option 1). Clang and GCC, in both versions, reject the initialization.

## Opinions
