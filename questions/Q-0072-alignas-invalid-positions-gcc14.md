# Q-0072: Can alignas apply to a function, a type alias, or a bit-field?

**Status:** settled
**Affects:** ALN-2, `tests/declarations/alignment/on-function.cpp`, `on-typedef.cpp`, `on-alias-declaration.cpp`, `on-bitfield.cpp`

## Context

GCC 14 accepts `alignas` on a function, a `typedef`, an alias declaration, and a bit-field. GCC 16 and the host GCC 16.2, Clang 18, 22, and the host Clang 22.1, and EDG 7.0 reject all four.

## Evidence

The four tests above, each with an override for GCC 14.

## Options

1. `alignas` must not occur in these positions.
2. `alignas` is valid there.

## Answer

No; GCC 14's acceptance is fixed in GCC 16 (option 1). The newest version of every compiler in the matrix rejects
all four. GCC 14 has a defect that GCC 16 fixes. The overrides for
GCC 14 stay while GCC 14 is in the matrix.

Settled on 2026-11-01 under decision 0051, because the newest version of each baseline family (Clang 22, GCC 16, EDG 7.0) agrees with this answer. GCC 14 still accepts all four, and the tests keep their overrides for GCC 14.

## Opinions
