# Q-0100: Is a character literal with value zero a null pointer constant?

**Status:** provisional
**Affects:** CONV-8, `tests/expressions/conversions/char-zero-not-null.cpp`

## Context

A pointer is initialized from a character literal whose value is zero. Clang and GCC reject it. EDG 7.0 accepts it as a null pointer.

## Options

1. Only an integer literal with value zero is a null pointer constant.

2. A character literal with value zero is one too.

## Provisional answer

Option 1. Clang and GCC agree.

## Opinions
