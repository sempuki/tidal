# Q-0159: Is `*this` the implied object argument when its class is not the class of the members found?

**Status:** provisional
**Affects:** OVL-2, OVL-3, `tests/foundations/overload-resolution/current-object-other-class.cpp`

## Context

A class `Outer` has a static `f(int)` and a non-static `f(long)`. A
member function of a nested class `Inner` calls `f(1L)`. Lookup finds
the members of `Outer`. `Inner` is not `Outer` and does not derive from
`Outer`.

| Compiler | Result |
|---|---|
| GCC 14, GCC 16 | rejects: `f(long)` is chosen and has no object |
| Clang 18, Clang 22 | accepts and calls the static `f(int)` |
| EDG 7.0 | accepts and calls the static `f(int)` |

Every compiler rejects the same call in a static member function of
`Outer`, and in a `static_assert` in the body of `Inner`, where no
`this` exists. There the non-static `f(long)` wins and has no object.

## Evidence

`tests/foundations/overload-resolution/current-object-other-class.cpp`.
Item E-2 of the gap review of overload resolution
(`reports/review/2026-10-31-gap-review-overload-resolution.md`)
describes the call as not valid.

## Options

1. The call has no object. The non-static member takes part with the
   exact match, wins, and makes the call not valid (GCC).
2. `*this` of `Inner` is the implied object argument. It cannot reach
   the object parameter of `Outer::f(long)`, so that function is not
   viable and the static member wins (Clang, EDG).

## Provisional answer

No, the call has no object; GCC agrees, Clang and EDG differ (option 1). The gap review describes it, and it gives the same result as
the calls with no `this`, on which every compiler agrees.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
