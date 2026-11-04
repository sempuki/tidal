# Q-0032: May the right operand of a compound assignment be a braced list?

**Status:** provisional
**Affects:** ASN-6, `tests/expressions/assignment/braced-compound.cpp`

## Context

`int x; x += {1};` uses a braced list as the right operand of `+=`. Clang
22 and 18 accept it without a diagnostic. GCC 16 and 14 ("invalid use of
brace-enclosed initializer list") and EDG 7.0 reject it. All compilers
accept a braced list on the right of simple `=`.

## Options

1. A braced list is allowed only as the right operand of simple `=`.
2. A braced list is also allowed for compound assignment to a scalar, as
   Clang does.

## Provisional answer

No; rejected (option 1). Two of three implementations reject a braced list with `+=`.

Kept open on 2026-11-03 under decision 0054, because the review finds the question unclear.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) finds the question unclear. The grammar lets every assignment operator, compound forms included, take a braced list as the right operand. For a scalar target a meaning is given only for simple assignment. For a class target the list goes to the selected operator function, and an example uses a compound operator on a class. The text does not say whether a compound assignment to a scalar counts here, or what such a form would mean. Implementations split, and the point is a known open issue. Option 1 needs no invented meaning, and Clang's reading is also possible.
