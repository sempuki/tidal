# Q-0071: Which alignas applies when a class has several?

**Status:** provisional
**Affects:** ALN-1, `tests/declarations/alignment/strictest-wins.cpp`

## Context

A class definition has several `alignas` specifiers with different values. Clang and EDG use the strictest. GCC 14, 16, and the host GCC 16.2 use the last one.

On a variable and on a non-static data member, GCC uses the strictest
of several `alignas`, as Clang and EDG do. `alignas(16) alignas(8)` on
either gives an alignment of 16 with every compiler.

## Evidence

`tests/declarations/alignment/strictest-wins.cpp`, with an override for
GCC.

## Options

1. The strictest alignment applies.
2. The last alignment applies.

## Provisional answer

Option 1. Clang and EDG agree, and every compiler, GCC included, uses
the strictest on variables and data members.

## Opinions
