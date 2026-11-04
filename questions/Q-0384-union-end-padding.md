# Q-0384: Does a union follow the end-padding conditions of a class?

**Status:** provisional
**Affects:** CLS-35, CLS-34, `tests/classes/members/layout-union-end-padding.cpp`, `tests/classes/members/layout-union-end-padding-ilp32.cpp`
**Source:** finding I1-10 of `reports/clean-room/I1/report.md`, from the probes that a clean-room writer ran for it.

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

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
