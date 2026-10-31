# Q-0143: Can a call through the address of an overload set choose a static member?

**Status:** provisional
**Affects:** OVL-21, `tests/foundations/overload-resolution/call-through-address-static-member.cpp`

## Context

A class has a non-static `reset()` and a static `reset(int)`. A member
function calls `(&Item::reset)(1)`.

| Compiler | Result |
|---|---|
| Clang 22 | accepts and calls the static member |
| Clang 18 | rejects: the overloaded function cannot be resolved |
| GCC 14, GCC 16 | rejects: no contextual type information |
| EDG 7.0 | stops with an internal error |

For free functions, `(&pick)(1)` works on every compiler
(`call-through-address.cpp`).

## Evidence

`tests/foundations/overload-resolution/call-through-address-static-member.cpp`.
Item AO-1 of the overload resolution inventory, and gap G-2, which says
that support for this form may be partial.

## Options

1. Resolution runs on the set with no implied object argument, and the
   static member wins (Clang 22).
2. The form is not valid for a set of members (Clang 18, GCC).

## Provisional answer

Option 1. The inventory describes the call this way, the newest Clang
implements it, and EDG's internal error is a defect.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
