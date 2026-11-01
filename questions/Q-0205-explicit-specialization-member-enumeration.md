# Q-0205: Can a member enumeration of a class template have an explicit specialization?

**Status:** provisional
**Affects:** TSP-30, TSP-6, TSP-3

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
override for GCC, and `explicit-enum-after-use.cpp`. The scoped and
defined case has no test.

## Options

1. A member enumeration that the class template only declares can have
   an explicit specialization, also after a use (Clang, EDG). An
   unscoped one that the class template defines cannot (all).
2. No member enumeration can have an explicit specialization (GCC in
   strict mode).

## Provisional answer

Yes if the class template only declares it; Clang and EDG accept, GCC rejects under -pedantic-errors (option 1). Clang and EDG agree. The scoped enumeration that the class
template defines stays open: Clang and EDG accept its explicit
specialization after a use, so Tidal states no rule for it.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
