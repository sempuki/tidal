# Q-0092: Do names in a lambda's trailing return type find its captures?

**Status:** provisional
**Affects:** LAM-18, `tests/expressions/lambdas/capture-visible-in-trailing-return.cpp`, `tests/expressions/lambdas/capture-not-visible-in-parameter-list.cpp`

## Context

A lambda copies `x` and has the trailing return type
`decltype((x))`. GCC 16 and EDG find the capture there, so the type is a
reference to `const int`. Clang 18 and 22 and GCC 14 find the outer
variable, so the type is `int&` and the body's return is not valid.

## Options

1. Captures are visible in the parameter list and trailing return type.
2. Names there refer to the enclosing scope.

## Provisional answer

Captures are visible in the trailing return type (option 1), and a name in the parameter list refers to the enclosing variable (option 2). GCC 16 and EDG find the capture in the trailing return type, and Clang does not. Every compiler finds the enclosing variable in the parameter list.

Changed on 2026-11-03 for the parameter list. The review reads option 2 there, and a probe on the matrix shows that every compiler agrees: `decltype((x))` as a parameter type is `int&`. LAM-P2 and LAM-18 now say that the trailing return type and the body see the captures, and that a name in the parameter list refers to the enclosing variable. The new test `capture-not-visible-in-parameter-list.cpp` accepts on every compiler. The trailing return type keeps option 1, which GCC 16 and EDG 7.0 follow and Clang 22 does not. The question stays open, because the rule changed on the same day.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1 for the trailing return type and reads option 2 for the parameter list. A name that would refer to a copy capture gets the type of the closure member in the lambda's own scope outside the parameter list, and the trailing return type is in that region. A name in the parameter list itself, such as in a parameter type or a default argument, refers to the enclosing variable. The test uses only the trailing return type, so its expected result agrees.
