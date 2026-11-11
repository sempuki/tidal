# Q-0410: In the definition of an enumeration with a qualified name, is the underlying type looked up in the scope that the qualifier names?

**Status:** provisional
**Affects:** NAM-21, `tests/foundations/names/qualified-enum-underlying-type-lookup.cpp`
**Source:** the gap review `reports/review/2026-11-05-gap-review-phase-9-declarations-statements-foundations.md`, on NAM-21, from the probes that a clean-room writer ran for it.
**Waits for:** the maintainer (pending.md row 4)

## Context

Namespace `N` declares `using T = short;` and `enum E : int;`. Outside `N`, the program defines `enum N::E : T { a };`. Clang 22 does not find `T`, GCC 16 does not parse the form, and EDG 7.0 rejects it. The gap review reads that everything after the qualified name, the underlying type included, is in the scope of `N`.

## Evidence

The test in **Affects**.

## Options

1. The definition is rejected (every baseline compiler).
2. `T` is found in `N` (the gap review).

## Provisional answer

Rejected (option 1). Every newest family rejects the definition. `pending.md` row 4 keeps the matrix's answer where at most one
newest family follows the review.

Kept in `pending.md` row 4 on 2026-11-10 (decision 0061, item RV6), because the review still reads against the answer.

## Opinions

### Gap review, 2026-11-05

The source-reader gap review (`reports/review/2026-11-05-gap-review-phase-9-declarations-statements-foundations.md`) reads the reference as option 2. See its section on NAM-21.

### Gap review, 2026-11-05 (phase 10 text)

The source-reader gap review of the text phase 10 wrote (`reports/review/2026-11-05-gap-review-phase-10-classes-declarations-foundations.md`) names the question in its section on NAM-21. The reference brings the underlying type into the scope of the qualifier, against the answer that every baseline compiler gives. The question stays in `pending.md` row 4.

### Gap review, 2026-11-05 (phase 12 text, classes, declarations, foundations, statements, and exceptions)

The source-reader gap review of the text phase 12 wrote on classes, declarations, foundations, statements, and exceptions (`reports/review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md`) names the question in its reading of NAM-21 (RV5-A-10). The underlying type of an enumeration comes after the qualified name, and the review still reads against the answer that the matrix keeps. The question stays in `pending.md` row 4.

### Gap review, 2026-11-10 (phase 13 text, classes, declarations, foundations, and statements)

The source-reader gap review of the text phase 13 wrote on classes, declarations, foundations, and statements (`reports/review/2026-11-10-gap-review-phase-13-classes-declarations-foundations.md`) names the question in its reading of NAM-21 (RV6-A-6). The underlying type of an enumeration comes after the qualified name, and the review still reads against the answer that the matrix keeps.
