# Q-0424: Does a member of an anonymous union whose type is an unnamed class defined in its own declaration count as a nested type?

**Status:** provisional
**Affects:** UNI-9, UNI-P3, `tests/classes/unions/anonymous-union-unnamed-struct-member.cpp`, `tests/classes/unions/anonymous-union-nested-anonymous-union.cpp`
**Source:** the round 10 programmer report (`reports/clean-room/roadmap-10/programmer.md`, sections 1.5 and 6), probe `anonymous_struct_member_in_anonymous_union.cpp`.
**Waits for:** a use of UNI-P3

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

An anonymous union inside an anonymous union also declares an unnamed
type. The writer probe
`reports/clean-room/roadmap-10/writer-probes/rv6-points.json` gives the
same split for that form: Clang rejects it under `-pedantic-errors` as
an extension, EDG rejects it, and GCC accepts it.

## Evidence

The tests in **Affects**. Each expects a rejection and records GCC as
an override.

## Options

1. The unnamed class is a type that the anonymous union declares, so
   UNI-9 rejects the program (Clang under `-pedantic-errors`, EDG).
2. Only a named nested type counts, and the program is valid (GCC).

## Provisional answer

Yes; the program is rejected (option 1). EDG rejects it, and Clang
marks it as an extension that strict mode rejects.

Rule text on 2026-11-10 (decision 0061, item F3): UNI-9's list of
nested types names an unnamed class that a member declaration defines.
The question waits for a first review.

Kept open on 2026-11-10 under decision 0054 (item RV6). The review supports the answer with no doubt, and only GCC differs. No principle gives the answer. UNI-P3 says an anonymous union holds only plain public data members, and `pair` is such a member. Both options fit UNI-P3, so UNI-P3 fails step 3 of the principle test.

Principle text on 2026-11-10 (decision 0061, item RV6): UNI-P3 says that an anonymous union declares no type, so a member whose declaration defines an unnamed class is not valid. UNI-P3 now rules out option 2. The principle changed in this phase, so it gives no answer until a reader or a review uses it (decision 0055).

## Opinions

### Gap review, 2026-11-10 (phase 13 text, classes, declarations, foundations, and statements)

The source-reader gap review of the text phase 13 wrote on classes, declarations, foundations, and statements (`reports/review/2026-11-10-gap-review-phase-13-classes-declarations-foundations.md`) reads the question: supports with no doubt (RV6-A-17). A member declaration that defines an unnamed class declares a type inside the anonymous union. Nested types are not allowed there, and nothing exempts an unnamed one, so the program is not valid and a diagnostic is required. GCC's acceptance is a missing diagnostic.
