# Q-0002: Does EDG accepting floating division by zero during constant evaluation change MUL-11?

**Status:** compiler defect
**Affects:** MUL-11, divergence MUL-D2, `tests/expressions/multiplicative/floating-divide-by-zero.cpp`, `tests/expressions/multiplicative/floating-zero-by-zero.cpp`

## Context

Clang and GCC reject `1.0 / 0.0` and `0.0 / 0.0` during constant evaluation.
EDG 7.0 accepts both. EDG rejects floating overflow (Q-0001), so it treats
some floating-point results as errors and not others.

## Evidence

`tests/RESULTS.md`, rows `floating-divide-by-zero.cpp` and
`floating-zero-by-zero.cpp`.

## Options

1. Keep MUL-11 as `undefined`, and report EDG's behavior to the EDG project
   as a possible defect.
2. Make floating division by zero `impl-defined`, with EDG's acceptance as
   one permitted outcome.

## Provisional answer

Option 1. Two of three implementations reject, and EDG's own handling is
inconsistent with its rejection of overflow.

## Opinions
