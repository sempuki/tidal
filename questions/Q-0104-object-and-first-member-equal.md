# Q-0104: Do a class object and its first member compare equal through void pointers?

**Status:** provisional
**Affects:** CMP-15, `tests/expressions/comparison/struct-and-first-member-equal.cpp`

## Context

A pointer to a class object and a pointer to its first non-static data member are converted to `const void*` and compared with `==`. Clang and GCC give `true` during constant evaluation. EDG 7.0 gives `false`.

## Options

1. The pointers are equal, because they hold the same address.

2. The result is unspecified.

## Provisional answer

Option 1. Clang and GCC agree, and CLS-10 gives the two the same address.

## Opinions
