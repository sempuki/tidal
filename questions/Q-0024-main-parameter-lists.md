# Q-0024: Which parameter lists may `main` have?

**Status:** provisional
**Affects:** FN-6, tests `main-*-param*.cpp` under `tests/declarations/functions/definitions`

## Context

| Parameters | Clang 22, 18 | GCC 16, 14 | EDG 7.0 |
|---|---|---|---|
| `()` | accept | accept | accept |
| `(int, char**)` | accept | accept | accept |
| `(int, char**, char**)` | accept | accept | accept |
| `(int)` | accept | reject (`-Wmain`, pedantic) | accept |
| `(double)` | reject | reject | accept |
| `(int, char**, char**, char**)` | reject | reject | accept |

## Evidence

`tests/RESULTS.md`, the `main-*` rows.

## Options

1. `()` and `(int, char**)` are required; every other list is
   implementation-defined.
2. Also require `(int, char**, char**)`, which all three accept.

## Provisional answer

Option 1. The three-parameter form is a common convention, but nothing in
the evidence shows it is more than each implementation's choice.

## Opinions
