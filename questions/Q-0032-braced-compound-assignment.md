# Q-0032: May the right operand of a compound assignment be a braced list?

**Status:** provisional
**Affects:** ASN-6, `tests/expressions/assignment/braced-compound.cpp`
**Waits for:** a review (doubt)

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

Kept open on 2026-11-11 under decision 0054 (decision 0066, item DQ). The second review supports the answer with a narrower doubt (DQ-3), and Clang 22 alone differs. The doubt is whether the scalar case of a braced right operand covers the compound operators.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) finds the question unclear. The grammar lets every assignment operator, compound forms included, take a braced list as the right operand. For a scalar target a meaning is given only for simple assignment. For a class target the list goes to the selected operator function, and an example uses a compound operator on a class. The text does not say whether a compound assignment to a scalar counts here, or what such a form would mean. Implementations split, and the point is a known open issue. Option 1 needs no invented meaning, and Clang's reading is also possible.

### Second review of the doubt-held questions, 2026-11-11

The source-reader second review of the questions held only by a doubt (`reports/review/2026-11-11-second-review-doubt-held-questions.md`) reads the question: supports with a doubt that remains (DQ-3). The grammar lets a braced list follow every assignment operator. The reference then gives the list a meaning in two places: a plain assignment to a scalar, and an assignment to a class, where the list goes to the chosen operator function. The doubt is now only whether the scalar case covers the compound operators. If the scalar case covers them, the reference gives the compound form no meaning. Option 1 needs no such meaning, and nothing in the reference reads against option 1.
