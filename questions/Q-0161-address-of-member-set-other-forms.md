# Q-0161: Can a call through `&(C::f)` or `&f` use a set that holds a non-static member?

**Status:** provisional
**Affects:** OVL-44, OVL-21, `tests/foundations/overload-resolution/call-through-address-parenthesized-qualified.cpp`, `tests/foundations/overload-resolution/call-through-address-unqualified.cpp`, `tests/foundations/overload-resolution/call-through-address-unqualified-argument.cpp`

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
`call-through-address-parenthesized-qualified.cpp`,
`call-through-address-unqualified.cpp`, and
`call-through-address-unqualified-argument.cpp`. Item E-7 of the gap review of
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

No for `&(C::f)`, yes for `&f`; the parenthesized form is not valid (option 1), and `&f` resolves the set as `&C::f` does (option 2).

Changed on 2026-11-03 for `&f`. The source-reader gap review of question cluster 2 (`reports/review/2026-11-03-gap-review-question-cluster-2.md`) agrees with option 1 for the parenthesized qualified form, and leans to option 2, with doubt, for the unqualified form. A parenthesized name of a member stands for a member access on the current object, and its address cannot be resolved. A name that is the direct operand of `&` is not rewritten, so `&f` is the address of the overload set. The review matches Clang 22 and EDG 7.0 on `&f`. The reading also fits OVL-21 and Q-0143, where a call through `&C::f` resolves the set with no implied object argument and the static member can win. MPTR-P1 and OVL-18 still hold, because the call makes no pointer to member. OVL-44 now covers only the parenthesized qualified form. `call-through-address-unqualified.cpp` expects the call to be accepted, with overrides for GCC and Clang 18, and `call-through-address-unqualified-argument.cpp` adds the form with an argument.

The question stays open under decision 0051 (2026-11-03). The newest versions split three ways. GCC 16 rejects every form. Clang 22 rejects the parenthesized form and accepts `&f`. EDG 7.0 accepts both forms with no argument and stops with an internal error with an argument. A probe of 2026-11-03 gives the table above.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 2 (`reports/review/2026-11-03-gap-review-question-cluster-2.md`) reads the reference as supporting option 1 for the parenthesized qualified form, and option 2, with doubt, for `&f`. For a call through the address of the set, the reference resolves the set with no implied object, and the call fails only when a member with an implicit object parameter wins. The doubt is a separate limit on where the name of such a member can appear, which the reference does not apply clearly to a mixed set whose chosen member is static.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
