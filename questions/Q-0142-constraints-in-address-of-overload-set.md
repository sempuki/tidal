# Q-0142: Do constraints reduce an overload set whose address is taken, with or without a target type?

**Status:** provisional
**Affects:** OVL-19, OVL-20, `tests/foundations/overload-resolution/address-more-constrained.cpp`, `tests/foundations/overload-resolution/address-no-target-constraints.cpp`, `tests/foundations/overload-resolution/address-no-target-more-constrained.cpp`, `tests/foundations/overload-resolution/address-no-target-constraints-different-parameters.cpp`

## Context

OVL-19 reduces the functions that the address of an overload set
finds. Step 2 removes a function whose constraints are not satisfied.
Step 4 removes a plain function if another plain function is more
constrained and the two meet the conditions of ORD-13. OVL-20 applies
steps 2 and 4 when there is no target type. Q-0146 holds the conditions
of ORD-13.

With a target type, a class template has two static members with the
type `int()`. The second has a constraint that subsumes the first
one's. The address of the overload set initializes a pointer of type
`int (*)()`.

| Compiler | Result |
|---|---|
| Clang 22, GCC 16 | the more constrained member |
| Clang 18, GCC 14 | ambiguous |
| EDG 7.0 | ambiguous |

Without a target type, a class template has two plain static members
`f()` with constraints. A program takes the address of `f` for one
specialization, as in `auto p = &Mode<1>::f;`.

| Compiler | Opposite constraints, `auto` | One subsumes the other, `auto` | One subsumes the other, `decltype` | One subsumes the other, different parameter lists, `auto` |
|---|---|---|---|---|
| Clang 22 | accepts | accepts | accepts | rejects |
| Clang 18 | accepts | rejects: ambiguous | accepts | accepts |
| GCC 16 | accepts | accepts | rejects | rejects |
| GCC 14 | rejects | rejects | rejects | rejects |
| EDG 7.0 | rejects | rejects | rejects | rejects |

The results are the same for a non-static member with `auto`. In a
call, every compiler chooses the more constrained member
(`tie-more-constrained.cpp`).

Q-0160 asked the case without a target type and is a duplicate of this
question.

## Evidence

The four tests in **Affects**, with overrides for the compilers that
differ. Item AO-3 of the overload resolution inventory describes step
4. Item E-6 of the gap review of overload resolution
(`reports/review/2026-10-31-gap-review-overload-resolution.md`)
describes both programs without a target type as valid.

## Options

1. A function whose constraints fail leaves the set, and a more
   constrained plain function removes a less constrained one when the
   two meet the conditions of ORD-13. The rule is the same with and
   without a target type (Clang 22, and GCC 16 except with `decltype`).
2. The set is ambiguous with a target type, and without a target type
   the set must have one function before any reduction (EDG, GCC 14).

## Provisional answer

Option 1. The newest versions of two implementations agree, the
inventory and the gap review describe the step, and the answer matches
the rule for calls (OVL-30, ORD-13).

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
