# Q-0205: Can a member enumeration of a class template have an explicit specialization?

**Status:** provisional
**Affects:** TSP-30, TSP-6, TSP-3, `tests/templates/specialization/explicit-enum-scoped-defined-after-use.cpp`, `tests/templates/specialization/explicit-enum-unscoped-declared-after-use.cpp`

## Context

A class template declares a member enumeration without enumerators,
such as `enum class Kind;`. After a use of `Holder<int>`, the program
gives `template <> enum class Holder<int>::Kind { c, d };`.

| Case | Clang 18, 22, host | GCC 14, 16, host | EDG 7.0 |
|---|---|---|---|
| declared only, after a use | accepts | rejects under `-pedantic-errors` | accepts |
| unscoped, defined in the class | rejects: specialization after instantiation | rejects under `-pedantic-errors` | rejects: the enumeration must be opaque |
| scoped, defined in the class, after a use | accepts | rejects under `-pedantic-errors` | accepts |

GCC rejects every explicit specialization of a member enumeration under
`-pedantic-errors`, and calls the form an extension.

## Evidence

`tests/templates/specialization/explicit-enum-declared.cpp`, with an
override for GCC, and `explicit-enum-after-use.cpp`.
`explicit-enum-scoped-defined-after-use.cpp` covers the scoped and
defined case, and `explicit-enum-unscoped-declared-after-use.cpp` an
unscoped enumeration with a fixed base that the class template only
declares. Clang 22 and EDG 7.0 accept both, and GCC 16 rejects both
under `-pedantic-errors`.

## Options

1. A member enumeration that the class template only declares can have
   an explicit specialization, also after a use (Clang, EDG). An
   unscoped one that the class template defines cannot (all).
2. No member enumeration can have an explicit specialization (GCC in
   strict mode).

## Provisional answer

Yes, also after a use, if the class template only declares it or if it is scoped; Clang and EDG accept, GCC rejects under -pedantic-errors (option 1). Clang and EDG agree. A use of the class makes only the declaration of a scoped member enumeration (INST-P2), so a scoped enumeration that the class template defines can have an explicit specialization after a use too. An unscoped one that the class template defines cannot.

Changed on 2026-11-03 for the case that was open. The review, Clang 22, and EDG 7.0 allow an explicit specialization of a scoped enumeration that the class template defines after a use, and INST-P2 gives the same answer: a use of the class makes only the declaration of a scoped member enumeration. TSP-30 now states this case, and TSP-6 names only the unscoped enumerations that the class template defines. The new test `explicit-enum-scoped-defined-after-use.cpp` has an override for GCC. For an unscoped enumeration that the class template only declares, the review reads a specialization after a use as not valid. A probe shows Clang 22 and EDG 7.0 accepting it, and GCC 16 rejecting every such specialization under `-pedantic-errors`. TSP-30 keeps that form valid until the maintainer decides (`explicit-enum-unscoped-declared-after-use.cpp`, row 4 of `documentation/decisions/pending.md`).

Kept open on 2026-11-03 under decision 0054, because the review reads against the answer for an unscoped enumeration that the class template only declares, and Tidal keeps that form valid until the maintainer decides (`documentation/decisions/pending.md`, row 4). TSP-30 also changed on the same day.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) agrees for the tested cases and reads against TSP-30 in part. Instantiating a class specialization makes only the declaration of a scoped member enumeration and the full definition of an unscoped one. So after a use, a scoped member enumeration can still have an explicit specialization, and an unscoped one cannot, even when the class template only declares it with a fixed base. GCC's pedantic error is a defect. The review also answers the case that was open: a scoped enumeration that the class template defines can have an explicit specialization after a use, while nothing has needed its definition.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
