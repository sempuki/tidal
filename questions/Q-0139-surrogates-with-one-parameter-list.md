# Q-0139: Is a call ambiguous when two surrogates differ only in the qualifiers of their conversion functions?

**Status:** provisional
**Affects:** OVL-6, OVL-36, `tests/foundations/overload-resolution/surrogate-same-parameters.cpp`

## Context

A class has `operator Fn()` and `operator Fn() const`, where `Fn` is a
pointer to function. Each gives a surrogate candidate with the same
parameter list. A call on an object of the class that is not `const`
must choose one surrogate.

| Compiler | Result |
|---|---|
| Clang 18, Clang 22 | ambiguous |
| EDG 7.0 | ambiguous |
| GCC 14, GCC 16 | accepts |

The object's sequences use different conversion functions, so they tie
(ICS-25). With conversions to two different function pointer
types, every compiler reports the tie (object-call grid, columns
"surrogate / const surrogate").

## Evidence

`tests/foundations/overload-resolution/surrogate-same-parameters.cpp`.
Item AF-7 of the overload resolution inventory.

## Options

1. The call is ambiguous (Clang, EDG).
2. The surrogate whose conversion function binds the object better
   wins (GCC).

## Provisional answer

Option 1. Two of three implementations agree, and the answer follows
the sequence rules of ICS-25 without an exception for surrogates.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
