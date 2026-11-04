# Q-0024: Which parameter lists may `main` have?

**Status:** settled
**Affects:** FN-6, TR-3, tests `main-*-param*.cpp` under `tests/declarations/functions/definitions`, `tests/foundations/translation/main-wrong-parameter-types.cpp`

## Context

| Parameters | Clang 22, 18 | GCC 16, 14 | EDG 7.0 |
|---|---|---|---|
| `()` | accept | accept | accept |
| `(int, char**)` | accept | accept | accept |
| `(int, char**, char**)` | accept | accept | accept |
| `(int)` | accept | reject (`-Wmain`, pedantic) | accept |
| `(double)` | reject | reject | accept |
| `(int, char**, char**, char**)` | reject | reject | accept |
| `(long, char**)` | reject | reject | accept |

TR-3 refers to FN-6 for the parameter lists of `main`. Q-0112 asked
about the `(long, char**)` row for TR-3 and is a duplicate of this
question.

## Evidence

`tests/RESULTS.md`, the `main-*` rows, and
`tests/foundations/translation/main-wrong-parameter-types.cpp`, which
has an override for EDG.

## Options

1. `()` and `(int, char**)` are required; every other list is
   implementation-defined.
2. Also require `(int, char**, char**)`, which all three accept.

## Answer

() and (int, char**) everywhere; others implementation-defined, including a first parameter of type `long` (option 1). The three-parameter form is a common convention, but nothing in
the evidence shows it is more than each implementation's choice.

Settled on 2026-11-03 under decision 0054. The review supports this answer with no doubt, and TR-P3 gives the same answer: every implementation accepts the two forms, and each chooses whether to accept others. EDG 7.0 accepts a first parameter of type `long`, which the answer allows, so no family differs.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of program structure (`reports/review/2026-11-03-gap-review-program-structure.md`) reads the reference as supporting option 1, the provisional answer, with no doubt. The reference requires the form with no parameters and the form with a count and an array of strings. It leaves the rest of the type of `main` to the implementation, and recommends that extra parameters come after the array.
