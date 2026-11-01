# Q-0258: Can a virtual function have a trailing requires clause?

**Status:** provisional
**Affects:** CPT-15, `tests/templates/concepts/virtual.cpp`, `tests/templates/concepts/virtual-override.cpp`

## Context

A member function of a class template has a trailing requires clause
and is virtual. In the second test, the function has no `virtual`, but
it overrides a virtual function of a base.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects: virtual function cannot have a requires clause |
| GCC 14, 16, host | accepts both |
| EDG 7.0 | rejects: a virtual function cannot have a trailing requires clause |

## Evidence

The two tests above. Clang's diagnostic list has a separate error for this case (clang-diag-sema).

## Options

1. Not valid (Clang, EDG). A call through a base class could reach a
   function whose constraints are not satisfied.
2. Valid (GCC).

## Provisional answer

No; Clang and EDG agree, GCC differs (option 1). Two of three implementations agree, and a virtual call has no point at which the constraints could be checked.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
