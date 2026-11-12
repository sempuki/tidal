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

Kept in `pending.md` row 4 on 2026-11-11 (decision 0065, item RV10). The gap review of the phase 17 text names the question at INST-30 and reads against the answer again (RV10-16). Every compiler still rejects the program.

## Opinions

### Gap review, 2026-11-05

The source-reader gap review (`reports/review/2026-11-05-gap-review-phase-9-expressions-templates.md`) reads the reference as option 2. See its section on PFX-P3, CLS-2.

### Gap review, 2026-11-05 (phase 10 text)

The source-reader gap review of the text phase 10 wrote (`reports/review/2026-11-05-gap-review-phase-10-classes-declarations-foundations.md`) names the question in its section on CLS-2. The reference counts a default template argument of a member template among the places where the class is complete, against the answer that every baseline compiler gives. The question stays in `pending.md` row 4.

### Gap review, 2026-11-11 (phase 17 text)

The source-reader gap review of the phase 17 text (`reports/review/2026-11-11-gap-review-phase-17.md`) names the question in its reading of INST-30 (RV10-16). INST-30's item for the operand of a `noexcept` specifier agrees, since the class is complete there in the reference. The reference also counts a default template argument inside the member list among the places where the class is complete. The INST Note says that INST-30 does not name that place, and follows CLS-2 and every compiler. The review reads that choice against the reference, as the earlier reviews did.
