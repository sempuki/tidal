# Q-0161: Can a call through `&(C::f)` or `&f` use a set that holds a non-static member?

**Status:** provisional
**Affects:** OVL-44, OVL-21, OVL-36, `tests/foundations/overload-resolution/call-through-address-parenthesized-qualified.cpp`, `tests/foundations/overload-resolution/call-through-address-unqualified.cpp`, `tests/foundations/overload-resolution/call-through-address-unqualified-argument.cpp`

## Context

A class `Item` has a non-static `f() &` and a static `f(int = 0)`. A
member function calls the set through its address, in one of these
forms.

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

No for `&(C::f)`, yes for `&f`; the parenthesized form is not valid (option 1), and `&f` resolves the set as `&C::f` does (option 2). With no argument, the two members tie, and the call `(&f)()` is ambiguous.

Changed on 2026-11-03 for `&f`. The source-reader gap review of question cluster 2 (`reports/review/2026-11-03-gap-review-question-cluster-2.md`) agrees with option 1 for the parenthesized qualified form, and leans to option 2, with doubt, for the unqualified form. A parenthesized name of a member stands for a member access on the current object, and its address cannot be resolved. A name that is the direct operand of `&` is not rewritten, so `&f` is the address of the overload set. The review matches Clang 22 and EDG 7.0 on `&f`. The reading also fits OVL-21 and Q-0143, where a call through `&C::f` resolves the set with no implied object argument and the static member can win. MPTR-P1 and OVL-18 still hold, because the call makes no pointer to member. OVL-44 now covers only the parenthesized qualified form. `call-through-address-unqualified.cpp` expects the call to be accepted, with overrides for GCC and Clang 18, and `call-through-address-unqualified-argument.cpp` adds the form with an argument.

Changed on 2026-11-03 for `(&f)()` with no argument. Front-end finding G103 (`reports/clean-room/rust-frontend/report-11.md`) shows that the text gives a tie there. OVL-21 resolves the set as a call by name with no implied object argument. OVL-23 leaves out the object parameter of `f() &`, and OVL-25 leaves out the parameter of `f(int = 0)` that takes the default. Neither member has a sequence to compare, no tie-breaker applies, and OVL-36 makes the call ambiguous. The same set called as `f()` with no object is ambiguous on every compiler (`no-object-static-default-ambiguous.cpp`), and Clang 18 reports the same ambiguity for `(&f)()`. A tie-breaker for the static member would split the call through the address from the call by name, and OVL-21 makes the two the same call. So the rules stay, and the expectation changes. `call-through-address-unqualified.cpp` now expects a rejection under OVL-36, with overrides for Clang 22 and EDG 7.0, which call the static member. With an argument, `(&f)(1)` still calls the static member, because only `f(int = 0)` takes one argument. The front end rejects `(&f)()`, so `tools/frontend_check.py` now counts the test as passing.

The question stays open under decision 0051 (2026-11-03). The newest versions split three ways. GCC 16 rejects every form. Clang 22 rejects the parenthesized form and accepts `&f`. EDG 7.0 accepts both forms with no argument and stops with an internal error with an argument. A probe of 2026-11-03 gives the table above.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 2 (`reports/review/2026-11-03-gap-review-question-cluster-2.md`) reads the reference as supporting option 1 for the parenthesized qualified form, and option 2, with doubt, for `&f`. For a call through the address of the set, the reference resolves the set with no implied object, and the call fails only when a member with an implicit object parameter wins. The doubt is a separate limit on where the name of such a member can appear, which the reference does not apply clearly to a mixed set whose chosen member is static.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
