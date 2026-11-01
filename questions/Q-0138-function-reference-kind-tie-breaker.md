# Q-0138: Does the kind of a function reference break a tie between two conversion functions?

**Status:** provisional
**Affects:** OVL-15, OVL-29, `tests/foundations/overload-resolution/tie-function-reference-kind.cpp`, `tests/foundations/overload-resolution/reference-function-both-kinds.cpp`

## Context

A class has two conversion functions to the same function type `F`. One
returns `F&` and the other returns `F&&`. A reference to `F` binds
directly to the result of either one, and both sequences are the
identity.

| Compiler | `F& r = Source{};` | `F&& r = Source{};` | `F& r` with only `operator F&&()` |
|---|---|---|---|
| GCC 14, GCC 16 | the `F&` conversion | the `F&&` conversion | accepts |
| EDG 7.0 | the `F&` conversion | the `F&&` conversion | accepts |
| Clang 18, Clang 22 | the `F&` conversion | ambiguous | rejects |

The last column shows that Clang does not give an lvalue reference to
function a conversion function that returns an rvalue reference. Clang
then chooses `F&` for an lvalue reference without a tie-breaker.

## Evidence

`tests/foundations/overload-resolution/tie-function-reference-kind.cpp`
and `reference-function-both-kinds.cpp`. The probe grid
`grids/reference-kind.json` covers every pair of the four conversion
functions `F&`, `F&&`, template `T&`, and template `T&&`. Item RK-4 of
the overload resolution inventory names this tie-breaker, and gap G-6
says that compilers can ignore it. Item O-1 of the gap review of
overload resolution
(`reports/review/2026-10-31-gap-review-overload-resolution.md`) also
describes the tie-breaker.

## Options

1. A reference to function takes conversion functions of both kinds,
   and the conversion that returns the target's kind of reference wins
   (GCC, EDG).
2. Each kind of reference takes only its own kind of conversion
   function, and no tie-breaker applies (Clang).

## Provisional answer

Yes; GCC and EDG agree, Clang differs (option 1). Two of three implementations agree, and the inventory and the
gap review describe the tie-breaker.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
