# Q-0014: May an implementation perform dynamic initialization before the program starts?

**Status:** provisional
**Affects:** SINIT-4, `tests/declarations/initialization/static/maybe-static.cpp`

## Context

`int b = three();`, where `three` is an `inline` function that returns 3
but isn't `constexpr`, is a dynamic initialization. An initializer earlier
in the same unit that reads `b` sees 0 if `b` is initialized at run time in
order, and 3 if it was initialized before the program started.

## Evidence

Clang 22 and GCC 16 at `-O0` and `-O2` give 0. GCC 16 with
`-fimplicit-constexpr` gives 3. The test accepts both.

## Options

1. `unspecified`: an implementation may do the initialization early when
   the result is the same. GCC's option is one such implementation choice.
2. `required` in-order initialization, which forbids GCC's option.

## Provisional answer

Option 1. The default behavior of every compiler fits it, and it doesn't
rule out an implementation choice that already exists.

## Opinions
