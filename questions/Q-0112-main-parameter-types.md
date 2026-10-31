# Q-0112: Must the parameters of main have the types int and char**?

**Status:** provisional
**Affects:** TR-3, `tests/foundations/translation/main-wrong-parameter-types.cpp`

## Context

A program defines `main` with a first parameter of type `long` instead of
`int`. Clang and GCC reject the definition. EDG 7.0 accepts it.

## Options

1. The two-parameter form of `main` must take `int` and `char**`.
2. Other parameter types are valid.

## Provisional answer

Option 1. Clang and GCC agree, and the runtime passes an `int`.

## Opinions
