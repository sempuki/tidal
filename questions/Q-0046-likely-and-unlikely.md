# Q-0046: Is `[[likely]] [[unlikely]]` on one statement rejected or ignored?

**Status:** provisional
**Affects:** ATTR-2, `tests/declarations/attributes/likely-and-unlikely.cpp`

## Context

Clang 22 and 18 and EDG 7.0 reject `likely` and `unlikely` on the same statement. GCC 16 and 14 accept it and ignore the second attribute with a warning.

## Options

1. Rejected.
2. Ignored, with a recommended warning.

## Provisional answer

Option 1. Two of three implementations reject it.

## Opinions
