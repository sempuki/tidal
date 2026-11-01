# Q-0061: Can a friend comparison declared outside the class be defaulted in it?

**Status:** provisional
**Affects:** DCMP-3, `tests/classes/comparisons/declared-outside-then-defaulted.cpp`

## Context

A program declares a comparison operator at namespace scope. Then a class
declares the same function as a friend and defaults it. Clang and EDG
reject the default. GCC accepts it.

## Options

1. The first declaration of a defaulted comparison must be in its class.
2. An earlier declaration outside the class is valid.

## Provisional answer

No; the first declaration of a defaulted comparison must be in its class (option 1). Clang and EDG agree.

## Opinions
