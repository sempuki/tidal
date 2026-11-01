# Q-0060: Can a literal operator have a default argument?

**Status:** provisional
**Affects:** OPR-8, `tests/classes/operators/literal-default-arg.cpp`

## Context

A literal operator for strings takes a pointer and a length. A program
can declare the length parameter with a default argument. A literal never
uses the default, because the implementation always passes the length.
Clang and EDG reject the default argument. GCC accepts it.

## Options

1. A literal operator must not have a default argument.
2. A default argument is valid and has no effect on literals.

## Provisional answer

No; a literal operator must not have a default argument (option 1). Clang and EDG agree, and other operator functions have the
same limit.

## Opinions
