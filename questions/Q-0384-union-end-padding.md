# Q-0384: Does a union follow the end-padding conditions of a class?

**Status:** provisional
**Affects:** CLS-35, CLS-34, `tests/classes/members/layout-union-end-padding.cpp`, `tests/classes/members/layout-union-end-padding-ilp32.cpp`
**Source:** finding I1-10 of `reports/clean-room/I1/report.md`, from the probes that a clean-room writer ran for it.
**Waits for:** a review (silent)

## Context

A union cannot be a base, so its end padding matters only for a
`[[no_unique_address]]` member of union type. Each union below has a
`char[5]` and an `int`, so it takes 8 bytes and its largest member
ends at byte 5. A class holds the union as a marked member and then
one `char`. A size of 12 means that the union keeps its end padding,
and 8 means that the `char` goes into it.

| The union has | Clang 22, GCC 16 | EDG 7.0 |
|---|---|---|
| no other declaration | 12 | 12 |
| a user-provided constructor | 8 | 12 |
| a user-provided destructor | 8 | 12 |
| a `private` member | 8 | 12 |
| a default member initializer | 8 | 12 |

The results are the same on x86-64 and with `-m32`. The use of end
padding is unspecified (CLS-34), so each answer is allowed. The
question records which choice CLS-35 lists for the matrix.

## Evidence

The tests in **Affects**, with an override for EDG.

## Options

1. A union keeps its end padding under the same conditions as a class
   (Clang, GCC).
2. A union always keeps its end padding (EDG).

## Provisional answer

The conditions of a class apply to a union (option 1). Two of three
families agree, and CLS-35 needs no separate list for unions. CLS-35
lists the EDG choice as an extra condition of EDG.

Kept open on 2026-11-04 under decision 0054, because the gap review of the text phase 8 wrote on classes and overloading finds the reference silent. Both options conform, and the question records a property of the platform ABIs. EDG 7.0 is the one newest family that differs.

## Opinions

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on classes and overloading (`reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`) finds the reference silent and supports neither option. The reference leaves the use of end padding to the implementation for unions as for other classes. Both options conform. The question records which choice the platform ABIs make, which CLS-35 lists.

### Gap review, 2026-11-04 (expressions, foundations, and templates)

The source-reader gap review of the text phase 8 wrote on expressions, foundations, exceptions, and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`) names the question and leaves it out, because its rules are on the classes pages.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
