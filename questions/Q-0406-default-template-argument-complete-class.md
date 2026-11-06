# Q-0406: Is a class complete in a default template argument of its member template?

**Status:** provisional
**Affects:** PFX-11, PFX-P3, CLS-2, `tests/expressions/postfix/default-template-argument-incomplete-class.cpp`
**Source:** the gap review `reports/review/2026-11-05-gap-review-phase-9-expressions-templates.md`, on PFX-P3, CLS-2, from the probes that a clean-room writer ran for it.
**Waits for:** the maintainer (pending.md row 4)

## Context

A member template of `S` has the default template argument `sizeof(S)`. Clang 22, GCC 16, and EDG 7.0 reject the use of `sizeof` on the incomplete class. The gap review lists the default template argument among the places where the class is complete.

## Evidence

The test in **Affects**.

## Options

1. The class is not complete there, and the program is rejected (every baseline compiler).
2. The class is complete there (the gap review).

## Provisional answer

Not complete (option 1). Every newest family rejects the program. `pending.md` row 4 keeps the matrix's answer where at most one
newest family follows the review.

## Opinions

### Gap review, 2026-11-05

The source-reader gap review (`reports/review/2026-11-05-gap-review-phase-9-expressions-templates.md`) reads the reference as option 2. See its section on PFX-P3, CLS-2.

### Gap review, 2026-11-05 (phase 10 text)

The source-reader gap review of the text phase 10 wrote (`reports/review/2026-11-05-gap-review-phase-10-classes-declarations-foundations.md`) names the question in its section on CLS-2. The reference counts a default template argument of a member template among the places where the class is complete, against the answer that every baseline compiler gives. The question stays in `pending.md` row 4.
