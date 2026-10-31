# Q-0052: Can a using-declaration make a public base member private?

**Status:** provisional
**Affects:** ACC-9, `tests/classes/access/using-reduces-access.cpp`

## Context

A derived class with a public base can redeclare a base member with a using-declaration under a `private` label. Clang and GCC then reject a use of that member through the derived class from outside. EDG 7.0 accepts the use. This result suggests that EDG keeps the access that the member has through the base.

## Options

1. The using-declaration sets the access, so the use is rejected.
2. The more permissive of the two accesses applies, so the use is accepted.

## Provisional answer

Option 1. Clang and GCC agree, and the same rule increases access in `using-changes-access.cpp` on all compilers.

## Opinions
