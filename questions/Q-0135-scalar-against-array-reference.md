# Q-0135: Does a braced list prefer a scalar parameter to a reference to an array when both sequences are the identity?

**Status:** settled
**Affects:** ICS-17, `tests/overloading/conversion-sequences/list-array-against-scalar.cpp`
**Parent:** Q-0294, the model of braced lists for references, arrays, and conversion sequences

## Context

The argument is `{1}`. One function takes `int`, and the other takes
`int (&&)[2]`. The sequence to `int` is the identity (ICS-15). The
sequence to the array is the identity for each element (ICS-17). No
tie-breaker on the page names this pair.

| Compiler | Result |
|---|---|
| Clang 18, Clang 22 | tie |
| EDG 7.0 | tie |
| GCC 14, GCC 16 | prefers `int` |

The lists grid shows the same split for `int (&&)[]`.

## Evidence

`tests/overloading/conversion-sequences/list-array-against-scalar.cpp`,
and the "one {1}" rows of the lists grid in `tests/overloading/conversion-sequences/grids/tables.md`.

## Options

1. The two sequences tie (Clang, EDG).
2. The sequence to the scalar is better (GCC). The page then needs a
   tie-breaker for this case.

## Answer

No, they tie; GCC differs (option 1). Two of three implementations agree, and the page's rules give
a tie with no further rule.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on overloading, references, and list initialization (`reports/review/2026-11-03-gap-review-questions-overloading.md`) supports this answer with no doubt, and ICS-P5 gives the same answer: step 2 compares only arrays of one element type or an initializer list, and when no step decides, the sequences tie. GCC 16 still differs. Tidal records GCC 16's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for GCC 16.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on overloading, references, and list initialization (`reports/review/2026-11-03-gap-review-questions-overloading.md`) agrees with option 1. The list tie-breaker compares two array targets, or an initializer list target with another target. A scalar target is neither, and no other rule separates two identity sequences when only one side binds a reference. GCC's choice is a defect.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
