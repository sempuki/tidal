# Q-0072: Can alignas apply to a function, a type alias, or a bit-field?

**Status:** provisional
**Affects:** ALN-2, `tests/declarations/alignment/on-function.cpp`, `on-typedef.cpp`, `on-alias-declaration.cpp`, `on-bitfield.cpp`

## Context

GCC 14 accepts `alignas` on a function, a `typedef`, an alias declaration, and a bit-field. GCC 16, Clang, and EDG reject all four.

## Options

1. `alignas` must not occur in these positions.
2. `alignas` is valid there.

## Provisional answer

Option 1. The newer GCC agrees with Clang and EDG.

## Opinions
