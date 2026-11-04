# Q-0159: Is `*this` the implied object argument when its class is not the class of the members found?

**Status:** provisional
**Affects:** OVL-2, OVL-3, `tests/overloading/overload-resolution/current-object-other-class.cpp`

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

`tests/overloading/overload-resolution/current-object-other-class.cpp`.
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

The question stays open under decision 0051 (2026-11-03). GCC 16 rejects the call, and Clang 22 and EDG 7.0 call the static `f(int)`, so two families disagree. The provisional answer follows OVL-P2 and OVL-3. `*this` is the implied object argument only for members of its class or of a base. Here the call has no object, and OVL-3 gives the result that every compiler gives in a static member function of `Outer`. Item E-2 of the gap review gives the same answer.

Kept open on 2026-11-03 under decision 0054, because two of the newest families differ from the answer in at least one form, and the rule records one differing family.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 2 (`reports/review/2026-11-03-gap-review-question-cluster-2.md`) agrees with option 1. `*this` is the implied object only when its class is the class of the members found or derives from that class. Otherwise the call uses a made-up object that every member accepts, and the call is not valid when a non-static member wins. Here the exact match wins, so the call is rejected. The implicit member access inside a member function would also fail, because the nested class is not related to the enclosing class.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
