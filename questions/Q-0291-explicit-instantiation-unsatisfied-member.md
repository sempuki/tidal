# Q-0291: Can a program explicitly instantiate a member function whose constraints fail for the class's arguments?

**Status:** settled
**Affects:** CNS-13, `tests/templates/constraints/explicit-instantiation-member-not-satisfied.cpp`

## Context

`Box<T>` has `void f() requires Wide<T>`, and the program writes
`template void Box<char>::f();`. `char` fails `Wide`.

| Compiler | Result |
|---|---|
| Clang 22, host | rejects: no viable candidate for explicit instantiation |
| Clang 18 | accepts |
| GCC 14, 16, host | rejects: does not match any template declaration |
| EDG 7.0 | rejects: declaration is incompatible |

## Evidence

The test above. Every compiler accepts the same explicit instantiation
for `int` (`explicit-instantiation-member.cpp`). The gap review of
concepts and constraints (item 36) expects the rejection.

## Options

1. Not valid (Clang 22, GCC, EDG).
2. Valid (Clang 18).

## Answer

No; every current compiler agrees, Clang 18 differs (option 1). Every current implementation agrees.

Settled on 2026-11-01 under decision 0051, because the newest version of each baseline family (Clang 22, GCC 16, EDG 7.0) agrees with this answer. Clang 18 still accepts the explicit instantiation, and the test keeps its override for Clang 18.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
