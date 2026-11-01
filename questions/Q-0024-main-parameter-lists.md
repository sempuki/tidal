# Q-0024: Which parameter lists may `main` have?

**Status:** provisional
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

## Provisional answer

() and (int, char**) everywhere; others implementation-defined, including a first parameter of type `long` (option 1). The three-parameter form is a common convention, but nothing in
the evidence shows it is more than each implementation's choice.

## Opinions
