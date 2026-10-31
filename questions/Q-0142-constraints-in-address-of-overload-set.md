# Q-0142: Does the more constrained function win when a target type selects from an overload set?

**Status:** provisional
**Affects:** OVL-19, `tests/foundations/overload-resolution/address-more-constrained.cpp`

## Context

A class template has two static members with the type `int()`. The
second has a constraint that subsumes the first one's. The address of
the overload set initializes a pointer of type `int (*)()`.

| Compiler | Result |
|---|---|
| Clang 22, GCC 16 | the more constrained member |
| Clang 18, GCC 14 | ambiguous |
| EDG 7.0 | ambiguous |

In a call, every compiler chooses the more constrained member
(`tie-more-constrained.cpp`).

## Evidence

`tests/foundations/overload-resolution/address-more-constrained.cpp`.
Item AO-3 of the overload resolution inventory.

## Options

1. The more constrained function remains (Clang 22, GCC 16).
2. The set is ambiguous (EDG, Clang 18, GCC 14).

## Provisional answer

Option 1. The newest versions of two implementations agree, the
inventory describes the step, and the answer matches the rule for
calls.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
