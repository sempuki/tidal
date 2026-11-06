# Q-0408: Does a default of a derived class template parameter that the base does not use let an inherited deduction guide succeed?

**Status:** provisional
**Affects:** CTAD-16, `tests/templates/class-template-deduction/inherited-guide-unused-parameter-default.cpp`
**Source:** the gap review `reports/review/2026-11-05-gap-review-phase-9-expressions-templates.md`, on CTAD-16, from the probes that a clean-room writer ran for it.
**Waits for:** the maintainer (pending.md row 4)

## Context

Class template `D<T, U = int>` derives from `B<T>` and inherits its constructors. `D d(1.5);` needs the inherited guide. GCC 16 and EDG 7.0 deduce `D<double, int>`. Clang 22 finds no viable guide. The gap review reads that the deducibility test of the inherited guide never deduces `U`, so the guide fails, default or not.

## Evidence

The test in **Affects**, with an override for Clang.

## Options

1. The default rescues the guide, and the declaration is valid (GCC, EDG).
2. The guide fails (Clang, the gap review).

## Provisional answer

The guide succeeds with the default (option 1). Two of three families agree. `pending.md` row 4 keeps the matrix's answer where at most one
newest family follows the review.

## Opinions

### Gap review, 2026-11-05

The source-reader gap review (`reports/review/2026-11-05-gap-review-phase-9-expressions-templates.md`) reads the reference as option 2. See its section on CTAD-16.

### Gap review, 2026-11-05 (phase 10 text)

The source-reader gap review of the text phase 10 wrote (`reports/review/2026-11-05-gap-review-phase-10-expressions-templates.md`) names the question in its section on CTAD-16. The reference reads that a default does not rescue the inherited guide, as Clang does, against the answer of GCC and EDG that Tidal keeps. The question stays in `pending.md` row 4.
