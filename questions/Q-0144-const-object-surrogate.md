# Q-0144: Does a const object get a surrogate from a conversion function that is not const?

**Status:** provisional
**Affects:** OVL-6, `tests/foundations/overload-resolution/surrogate-qualifiers.cpp`

## Context

A class has `operator Fn()`, which is not `const`, and a `const` call
operator that takes `long`. A `const` object of the class is called with
an `int`.

| Compiler | Result |
|---|---|
| Clang 18, Clang 22 | the call operator |
| EDG 7.0 | the call operator |
| GCC 14, GCC 16 | ambiguous between the call operator and the surrogate |

With no call operator, GCC chooses the surrogate and then rejects the
`const` object for the conversion function.

## Evidence

`tests/foundations/overload-resolution/surrogate-qualifiers.cpp`, and
the rows "const S lvalue" of the object-call grid. Items CC-8 and gap
G-10 of the overload resolution inventory.

## Options

1. A conversion function gives a surrogate only if its cv-qualifiers
   include those of the object (Clang, EDG).
2. Every conversion function gives a surrogate (GCC).

## Provisional answer

No; Clang and EDG agree, GCC differs (option 1). Two of three implementations agree, and the inventory states
the condition.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
