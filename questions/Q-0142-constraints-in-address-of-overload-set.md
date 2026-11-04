# Q-0142: Do constraints reduce an overload set whose address is taken, with or without a target type?

**Status:** provisional
**Affects:** OVL-19, OVL-20, `tests/overloading/overload-resolution/address-more-constrained.cpp`, `tests/overloading/overload-resolution/address-no-target-constraints.cpp`, `tests/overloading/overload-resolution/address-no-target-more-constrained.cpp`, `tests/overloading/overload-resolution/address-no-target-constraints-different-parameters.cpp`

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

Yes, when the functions meet the conditions of ORD-13; Clang 22 and GCC 16 agree, EDG, Clang 18 and GCC 14 differ (option 1). The newest versions of two implementations agree, the
inventory and the gap review describe the step, and the answer matches
the rule for calls (OVL-30, ORD-13).

Kept on 2026-11-03. The review agrees for equal parameter lists. For different parameter lists without a target, Clang 22, GCC 16, and EDG 7.0 all reject the program, and only Clang 18 gives the review's result. Tidal keeps the conditions of ORD-13 in OVL-19, as for calls (OVL-30), until the maintainer decides. That form is in row 4 of `documentation/decisions/pending.md`. The question stays open under decision 0054, because the review reads against part of the answer and two newest families differ in the forms with a target.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on overloading, references, and list initialization (`reports/review/2026-11-03-gap-review-questions-overloading.md`) reads option 1 without the conditions of ORD-13. With a target and equal parameter lists, the more constrained function wins, as the answer says. The removal of a less constrained plain function has no condition on the parameter lists or the target type, so without a target and with different parameter lists the more constrained function remains alone. That is Clang 18's result, and the test that expects a rejection does not follow the review. The review has not checked whether a later change adds the conditions.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Inventory, 2026-10-31

The source-reader inventory of concepts and constraints (`reports/review/2026-10-31-inventory-concepts-and-constraints.md`) names the question in its list of compiler differences that Tidal already records, and gives no reading.
