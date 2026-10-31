# Q-0003: Does EDG accepting `INT_MIN / -1` during constant evaluation change MUL-10?

**Status:** compiler defect
**Affects:** MUL-10, divergence MUL-D3, `tests/expressions/multiplicative/signed-overflow-divide.cpp`, `tests/expressions/multiplicative/signed-overflow-remainder.cpp`

## Context

Clang and GCC reject `INT_MIN / -1` and `INT_MIN % -1` during constant
evaluation. EDG 7.0 accepts both. EDG rejects `INT_MAX * 2`, so it detects
signed overflow in multiplication but not in this division case.

## Evidence

`tests/RESULTS.md`, rows `signed-overflow-divide.cpp` and
`signed-overflow-remainder.cpp`.

## Options

1. Keep MUL-10 as `undefined`, and report EDG's behavior to the EDG project
   as a possible defect.
2. Treat `INT_MIN % -1` separately, since its mathematical result (zero) is
   representable.

## Provisional answer

Option 1 for both operators. Clang and GCC treat `%` the same as `/` here.

## Opinions
