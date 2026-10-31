# Q-0059: Can a conversion function have a deduced return type?

**Status:** provisional
**Affects:** CVF-1, `tests/classes/conversion-functions/deduced-auto.cpp`

## Context

A conversion function can be written as `operator auto()`, with the
target type deduced from its `return` statements. Clang 18 and 22, EDG
7.0, and GCC 14 accept it. GCC 16 rejects it under `-pedantic-errors`
and accepts it otherwise.

## Options

1. A conversion function can have a deduced return type.
2. The target type must be written.

## Provisional answer

Option 1. Most implementations accept `operator auto()`, and GCC 16
rejects it only as a pedantic diagnostic.

## Opinions
