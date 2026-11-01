# Q-0187: In a call with no object, does a member's object parameter take part in overload resolution?

**Status:** provisional
**Affects:** OVL-23, OVL-25, OVL-3, `tests/foundations/overload-resolution/no-object-ref-qualified-member.cpp`

## Context

A call by name in a static member function can find members with an
implicit object parameter. The call has no object (OVL-3). A class has
`static void put(long)` and `void put(int) &&`, and a static member
function calls `put(1)`.

| Compiler | Object of the call | Result |
|---|---|---|
| Clang 18, Clang 22 | none; the object parameter takes no part | `put(int) &&` wins, and the call is rejected for lack of an object |
| GCC 14, GCC 16 | an lvalue of the class | `put(int) &&` is not viable, and `put(long)` wins |
| EDG 7.0 | none; the object parameter takes no part | `put(int) &&` wins, and the call is rejected for lack of an object |

For `void k(int) &` and `void k(int) &&` with no object, EDG reports an
ambiguous call. Clang and GCC report a call without an object. GCC names
`k(int) &`.

## Evidence

`tests/foundations/overload-resolution/no-object-ref-qualified-member.cpp`.
The same split appears for `S::put(1)` at namespace scope: GCC accepts
the call, and Clang and EDG reject it.

## Options

1. The object parameter takes no part, whatever its cv-qualifiers and
   ref-qualifier (Clang, EDG).
2. The call has an lvalue object of the class, so the qualifiers can
   make a member not viable (GCC).

## Provisional answer

No; it takes no part; Clang and EDG agree, GCC differs (option 1). Two of three implementations agree. The answer also matches
ICS-14, where the object slot of a static member takes no part.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
