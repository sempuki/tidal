# Q-0068: Can a function be declared inline after its definition?

**Status:** provisional
**Affects:** INL-2, `tests/declarations/inline/inline-after-definition.cpp`

## Context

A program defines a function without `inline`, then declares it again
with `inline`. Clang and EDG reject the second declaration. GCC accepts
it.

## Options

1. A function must not be declared `inline` after its definition.
2. A later `inline` declaration is valid.

## Provisional answer

No; a function must not be declared `inline` after its definition (option 1). Clang and EDG agree.

## Opinions
