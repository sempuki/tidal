# Q-0072: Can alignas apply to a function, a type alias, or a bit-field?

**Status:** compiler defect
**Affects:** ALN-2, `tests/declarations/alignment/on-function.cpp`, `on-typedef.cpp`, `on-alias-declaration.cpp`, `on-bitfield.cpp`

## Context

GCC 14 accepts `alignas` on a function, a `typedef`, an alias declaration, and a bit-field. GCC 16 and the host GCC 16.2, Clang 18, 22, and the host Clang 22.1, and EDG 7.0 reject all four.

## Evidence

The four tests above, each with an override for GCC 14.

## Options

1. `alignas` must not occur in these positions.
2. `alignas` is valid there.

## Provisional answer

No; GCC 14's acceptance is fixed in GCC 16 (option 1). The newest version of every compiler in the matrix rejects
all four. GCC 14 has a defect that GCC 16 fixes. The overrides for
GCC 14 stay while GCC 14 is in the matrix.

## Opinions
