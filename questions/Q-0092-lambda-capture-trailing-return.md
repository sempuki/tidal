# Q-0092: Do names in a lambda's trailing return type find its captures?

**Status:** provisional
**Affects:** LAM-18, `tests/expressions/lambdas/capture-visible-in-trailing-return.cpp`

## Context

A lambda copies `x` and has the trailing return type
`decltype((x))`. GCC 16 and EDG find the capture there, so the type is a
reference to `const int`. Clang 18 and 22 and GCC 14 find the outer
variable, so the type is `int&` and the body's return is not valid.

## Options

1. Captures are visible in the parameter list and trailing return type.
2. Names there refer to the enclosing scope.

## Provisional answer

Option 1. GCC 16 and EDG agree, and it matches the
lookup inside the body.

## Opinions
