# Q-0409: Is an injected class name that two base classes give ambiguous as a template template argument?

**Status:** provisional
**Affects:** TARG-27, `tests/templates/arguments/injected-name-two-bases-template-argument.cpp`
**Source:** the gap review `reports/review/2026-11-05-gap-review-phase-9-expressions-templates.md`, on TARG-27, from the probes that a clean-room writer ran for it.
**Waits for:** the maintainer (pending.md row 4)

## Context

Class `D` derives from `B<int>` and `B<long>` and passes `B` as a template template argument. Clang 22, GCC 16, and EDG 7.0 reject the name as ambiguous. The gap review reads that the name means the template `B` in that use, with no ambiguity.

## Evidence

The test in **Affects**.

## Options

1. Ambiguous, and rejected (every baseline compiler).
2. The template `B`, and valid (the gap review).

## Provisional answer

Ambiguous (option 1). Every newest family rejects the program. `pending.md` row 4 keeps the matrix's answer where at most one
newest family follows the review.

## Opinions

### Gap review, 2026-11-05

The source-reader gap review (`reports/review/2026-11-05-gap-review-phase-9-expressions-templates.md`) reads the reference as option 2. See its section on TARG-27.

### Gap review, 2026-11-05 (phase 10 text)

The source-reader gap review of the text phase 10 wrote (`reports/review/2026-11-05-gap-review-phase-10-expressions-templates.md`) names the question in its section on TARG-27. The reference treats the name as the template, with no ambiguity, against the answer that every baseline compiler gives. The question stays in `pending.md` row 4.
