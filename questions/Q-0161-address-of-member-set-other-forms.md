# Q-0161: Can a call through `&(C::f)` or `&f` use a set that holds a non-static member?

**Status:** provisional
**Affects:** OVL-44, `tests/foundations/overload-resolution/call-through-address-parenthesized-qualified.cpp`, `tests/foundations/overload-resolution/call-through-address-unqualified.cpp`

## Context

A class `Item` has a non-static `f() &` and a static `f(int = 0)`. A
member function calls the set through its address with no argument, in
one of these forms. The static member would win the call.

| Compiler | `(&(Item::f))()` | `(&f)()` | `(&(Item::f))(1)` | `(&f)(1)` |
|---|---|---|---|---|
| GCC 14, GCC 16 | rejects | rejects | rejects | rejects |
| Clang 18 | rejects | rejects: ambiguous | rejects | accepts |
| Clang 22 | rejects | accepts | rejects | accepts |
| EDG 7.0 | accepts | accepts | internal error | internal error |

Clang reports that it cannot form a pointer to member function from
the parenthesized name. With `&Item::f`, Q-0143 records the results.

## Evidence

`tests/foundations/overload-resolution/call-through-address-parenthesized-static.cpp`,
`call-through-address-parenthesized-qualified.cpp`, and
`call-through-address-unqualified.cpp`. Item E-7 of the gap review of
overload resolution
(`reports/review/2026-10-31-gap-review-overload-resolution.md`)
describes both forms as not valid.

## Options

1. Only `&C::f` can take the address of a set with a non-static member.
   The other forms are not valid (GCC, and Clang for the parenthesized
   form).
2. Resolution runs on the set as for `&C::f`, and the static member
   wins (EDG, and Clang 22 for `&f`).

## Provisional answer

No; GCC agrees, Clang 22 and EDG differ in part (option 1). GCC rejects every form, Clang rejects the parenthesized form,
and the gap review describes the forms as not valid.

The question stays open under decision 0051 (2026-11-03). The newest versions split three ways. GCC 16 rejects every form. Clang 22 rejects the parenthesized form and accepts `&f`. EDG 7.0 accepts both forms with no argument and stops with an internal error with an argument. A probe of 2026-11-03 gives the table above. The provisional answer follows MPTR-P1, where only the form `&C::m` makes a pointer to member, and OVL-18, where a member with an implicit object parameter matches only a pointer to member. So `&f` and `&(C::f)` cannot take that member, whichever function of the set would win. Item E-7 of the gap review gives the same answer.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
