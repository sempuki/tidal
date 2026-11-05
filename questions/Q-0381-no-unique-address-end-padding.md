# Q-0381: Does a `[[no_unique_address]]` member make a class lend its end padding?

**Status:** provisional
**Affects:** CLS-35, CLS-34, `tests/classes/members/layout-no-unique-address-end-padding.cpp`, `tests/classes/members/layout-no-unique-address-end-padding-ilp32.cpp`
**Source:** finding I1-1 of `reports/clean-room/I1/report.md`, from the probes that a clean-room writer ran for it.

## Context

Each class below meets the end-padding conditions of CLS-35 and has an
`int`, a `char`, and a `[[no_unique_address]]` member. The class takes
8 bytes and ends with 3 bytes of padding. A derived class adds one
`char`. A size of 12 means that the base keeps its end padding, and 8
means that the derived member goes into it.

| Marked member | Clang 22 | GCC 14, GCC 16 | EDG 7.0 |
|---|---|---|---|
| an empty class member | 12 | 8 | 12 |
| a `char` member | 12 | 8 | 12 |

The results are the same on x86-64 and with `-m32`. On GCC, a marked
member of a class type that keeps its own end padding gives the same
size either way, because that member fills the class to its end.

The use of the end padding of a base is unspecified (CLS-34), so each
answer is allowed. The question records which choice CLS-35 lists for
the matrix.

## Evidence

The tests in **Affects**, with an override for GCC.

## Options

1. The attribute does not change whether the class keeps its end
   padding (Clang, EDG).
2. A class with a `[[no_unique_address]]` member lends its end padding
   (GCC).

## Provisional answer

The attribute does not change the end padding (option 1). Two of three
families agree. CLS-35 lists the GCC choice as an extra condition of
GCC.

Kept open on 2026-11-04 under decision 0054, because no review has read the question. Item RV2 of decision 0056 reads it.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
