# Q-0073: Can alignas apply to an enumeration?

**Status:** provisional
**Affects:** ALN-1, `tests/declarations/alignment/enum.cpp`

## Context

`enum alignas(8) E { a };` puts `alignas` on an enumeration. Clang and GCC 16 reject the declaration. EDG 7.0 and GCC 14 accept it.

## Options

1. `alignas` must not apply to an enumeration.
2. `alignas` on an enumeration is valid and sets the alignment of the
   enumeration.

## Provisional answer

Option 1. Clang and the newer GCC agree. The provisional answer has the weakest support of the group, and an opinion is welcome.

## Opinions
