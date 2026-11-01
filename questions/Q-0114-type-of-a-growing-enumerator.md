# Q-0114: What type does an enumerator get when its value outgrows the type before it?

**Status:** provisional
**Affects:** ENUM-13, `tests/declarations/enumerations/enumerator-type-grows.cpp`

## Context

In an enumeration without a fixed type, an enumerator without an
initializer takes the value of the one before it plus one. If the first
enumerator is the largest `int`, the second does not fit in `int`. Inside
the enumerator list, Clang gives the second enumerator the type `long`,
GCC gives it `unsigned int`, and EDG gives it an unsigned type of 8
bytes.

## Options

1. The type is unspecified: any integer type that can hold the value.
2. The type is the first type, by rank from `int` upward, that can hold
   the value, which matches Clang.

## Provisional answer

Unspecified: any integer type that holds the value (option 1). The three compilers give three types, and each type holds the
value.

## Opinions
