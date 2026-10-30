# Q-0001: Is floating overflow during constant evaluation undefined or implementation-defined?

**Status:** provisional
**Affects:** MUL-12, divergence MUL-D1, `tests/expressions/multiplicative/floating-overflow-constant.cpp`

## Context

`1e308 * 10.0` overflows `double`. At run time on IEEE 754 targets the
result is positive infinity. During constant evaluation the implementations
disagree.

| Compiler | Result |
|---|---|
| Clang 22.1.8, 18.1.3 | accepts; the value is positive infinity |
| GCC 16.2.1, 14.2.0 | rejects: "`(1.0e+308 * 1.0e+1)` is not a constant expression" |
| EDG 7.0 | rejects: "floating-point error" |

## Evidence

`tests/RESULTS.md`, row `floating-overflow-constant.cpp`.

## Options

1. `undefined`. GCC and EDG conform. Clang's acceptance is an extension.
2. `impl-defined`. All three conform. Programs can't rely on either outcome
   without checking each implementation's documentation.

## Provisional answer

Option 1. Two of three implementations reject, and a program that relies on
acceptance is not portable today.

## Opinions
