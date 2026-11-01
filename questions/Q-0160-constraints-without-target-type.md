# Q-0160: Do constraints reduce an overload set whose address has no target type?

**Status:** provisional
**Affects:** OVL-20, `tests/foundations/overload-resolution/address-no-target-constraints.cpp`, `tests/foundations/overload-resolution/address-no-target-more-constrained.cpp`

## Context

A class template has two plain static members `f()` with constraints.
A program takes the address of `f` for one specialization and has no
target type, as in `auto p = &Mode<1>::f;`.

| Compiler | Opposite constraints, `auto` | One subsumes the other, `auto` | One subsumes the other, `decltype` |
|---|---|---|---|
| Clang 22 | accepts | accepts | accepts |
| Clang 18 | accepts | rejects: ambiguous | accepts |
| GCC 16 | accepts | accepts | rejects |
| GCC 14 | rejects | rejects | rejects |
| EDG 7.0 | rejects | rejects | rejects |

The results are the same for a non-static member with `auto`. With a
target type, Q-0142 records the same question.

## Evidence

`tests/foundations/overload-resolution/address-no-target-constraints.cpp`
and `address-no-target-more-constrained.cpp`. Item E-6 of the gap review
of overload resolution
(`reports/review/2026-10-31-gap-review-overload-resolution.md`)
describes both programs as valid.

## Options

1. A function whose constraints fail leaves the set, and a more
   constrained plain function removes a less constrained one (Clang 22,
   GCC 16 with `auto`).
2. Without a target type the set must have one function before any
   reduction (GCC 14, EDG).

## Provisional answer

Option 1. The newest versions of two implementations agree, the gap
review describes it, and the answer matches Q-0142 and the rule for
calls.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
