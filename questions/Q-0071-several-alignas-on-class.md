# Q-0071: Which alignas applies when a class has several?

**Status:** provisional
**Affects:** ALN-1, `tests/declarations/alignment/strictest-wins.cpp`

## Context

A class definition has several `alignas` specifiers with different values. Clang and EDG use the strictest. GCC uses the last one.

## Options

1. The strictest alignment applies.
2. The last alignment applies.

## Provisional answer

Option 1. Clang and EDG agree, and the same rule applies to variables.

## Opinions
