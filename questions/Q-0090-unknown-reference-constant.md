# Q-0090: Can constant evaluation use a reference whose referent is unknown, if it does not read through it?

**Status:** provisional
**Affects:** CE-18, `tests/expressions/constant-expressions/unknown-reference-not-read.cpp`

## Context

A function that is not `constexpr` takes a reference parameter
and uses it to call a `constexpr` member function that does not read the
object, in the initializer of a `constexpr` variable. Clang 22, GCC 14
and 16, and EDG accept the program. Clang 18 rejects it.

## Options

1. The use is valid when the evaluation does not read through the
   reference.
2. Any use of such a reference makes the expression not constant.

## Provisional answer

Yes, when the evaluation does not read through the reference (option 1). The newer Clang agrees with GCC and EDG.

## Opinions
