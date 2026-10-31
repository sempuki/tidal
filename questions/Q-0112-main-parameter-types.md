# Q-0112: Must the parameters of main have the types int and char**?

**Status:** provisional
**Affects:** TR-3, FN-6, `tests/foundations/translation/main-wrong-parameter-types.cpp`

## Context

A program defines `main` with a first parameter of type `long` instead of
`int`. Clang and GCC reject the definition. EDG 7.0 accepts it.

## Options

1. The two-parameter form of `main` must take `int` and `char**`.
2. Other parameter types are valid.

## Provisional answer

Option 2, as FN-6 states it: an implementation accepts `()` and
`(int, char**)`, and decides about other parameter lists. Clang and GCC
reject a first parameter of type `long`, and EDG accepts it. TR-3 used
to require option 1, which contradicted FN-6; the clean-room front end
found the contradiction (`reports/clean-room/rust-frontend/`, F16).

## Opinions
