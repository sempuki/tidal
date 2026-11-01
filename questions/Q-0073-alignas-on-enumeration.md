# Q-0073: Can alignas apply to an enumeration?

**Status:** provisional
**Affects:** ALN-7, `tests/declarations/alignment/enum.cpp`

## Context

`enum alignas(8) E { a };` puts `alignas` on an enumeration. Clang 18 and 22 and GCC 16 reject the declaration. EDG 7.0 and GCC 14 accept it. The host GCC 16.2 and Clang 22.1 reject it too.

GCC changed between versions 14 and 16, so GCC 14's acceptance looks
like a defect. EDG 7.0 is the newest EDG in the matrix, and the split
with EDG stays.

## Evidence

`tests/declarations/alignment/enum.cpp`, with overrides for EDG and
GCC 14.

## Options

1. `alignas` must not apply to an enumeration.
2. `alignas` on an enumeration is valid and sets the alignment of the
   enumeration.

## Provisional answer

Option 1. Clang and the newer GCC agree. The provisional answer has the weakest support of the group, and an opinion is welcome.

## Opinions
