# Q-0083: Can a default member initializer of a nested class be used before the enclosing class is complete?

**Status:** provisional
**Affects:** CTOR-18, `tests/classes/constructors/nested-default-member-init-before-complete.cpp`

## Context

A class has a nested class with a default member initializer.
A default argument of a constructor of the enclosing class value-initializes
the nested class with empty braces. Clang and GCC reject the program. EDG
accepts it.

## Options

1. The program is not valid.
2. The program is valid.

## Provisional answer

No; the program is not valid (option 1). Clang and GCC agree.

## Opinions
