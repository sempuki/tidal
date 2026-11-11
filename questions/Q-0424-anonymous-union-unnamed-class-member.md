# Q-0424: Does a member of an anonymous union whose type is an unnamed class defined in its own declaration count as a nested type?

**Status:** provisional
**Affects:** UNI-9, `tests/classes/unions/anonymous-union-unnamed-struct-member.cpp`
**Source:** the round 10 programmer report (`reports/clean-room/roadmap-10/programmer.md`, sections 1.5 and 6), probe `anonymous_struct_member_in_anonymous_union.cpp`.
**Waits for:** a first review; a rule

## Context

A class declares the anonymous union
`union { struct { int key; int value; } pair; };`. The member
declaration of `pair` defines an unnamed class type. UNI-9 forbids
nested types in an anonymous union, and its test
`anonymous-union-nested-type.cpp` names the nested type.

| Form | Clang 18 and 22 | GCC 14 and 16 | EDG 7.0 | Tidal front end |
|---|---|---|---|---|
| an unnamed class as the type of a member | rejects under `-pedantic-errors` | accepts under `-pedantic-errors` | rejects | runs |

Clang calls the form an extension, so its rejection comes from
`-pedantic-errors`. EDG says that types cannot be declared in anonymous
unions.

## Evidence

The test in **Affects**. It expects a rejection and records GCC as an
override.

## Options

1. The unnamed class is a type that the anonymous union declares, so
   UNI-9 rejects the program (Clang under `-pedantic-errors`, EDG).
2. Only a named nested type counts, and the program is valid (GCC).

## Provisional answer

Yes; the program is rejected (option 1). EDG rejects it, and Clang
marks it as an extension that strict mode rejects.

## Opinions
