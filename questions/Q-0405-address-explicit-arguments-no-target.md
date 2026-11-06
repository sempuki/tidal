# Q-0405: With no target type, does partial ordering choose among the specializations that `&f<int>` names?

**Status:** provisional
**Affects:** ORD-14, ORD-P1, OVL-20, `tests/templates/ordering/address-explicit-arguments-no-target.cpp`
**Source:** the gap review `reports/review/2026-11-05-gap-review-phase-9-expressions-templates.md`, on ORD-P1, from the probes that a clean-room writer ran for it.
**Waits for:** the maintainer (pending.md row 4)

## Context

Two templates `f(T)` and `f(T*)` give the specializations `f<int>(int)` and `f<int>(int*)` for `&f<int>`. With no target type, Clang 22, GCC 16, and EDG 7.0 reject the address as ambiguous. The gap review reads that partial ordering of the function types keeps the more specialized `f(T*)`.

## Evidence

The test in **Affects**.

## Options

1. Ambiguous, and rejected (every baseline compiler).
2. `f<int>(int*)`, by partial ordering (the gap review).

## Provisional answer

Ambiguous (option 1). Every newest family rejects the address. `pending.md` row 4 keeps the matrix's answer where at most one
newest family follows the review.

## Opinions

### Gap review, 2026-11-05

The source-reader gap review (`reports/review/2026-11-05-gap-review-phase-9-expressions-templates.md`) reads the reference as option 2. See its section on ORD-P1.

### Gap review, 2026-11-05 (phase 10 text)

The source-reader gap review of the text phase 10 wrote (`reports/review/2026-11-05-gap-review-phase-10-expressions-templates.md`) names the question in its section on ORD-P1. The reference keeps partial ordering on the function types with no target, against the answer that every baseline compiler gives. A later defect resolution may change the answer. The question stays in `pending.md` row 4.
