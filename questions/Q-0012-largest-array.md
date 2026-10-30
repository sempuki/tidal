# Q-0012: Should Tidal set a minimum for the largest array an implementation accepts?

**Status:** provisional
**Affects:** DCL-10, `tests/declarations/declarators/array-too-large.cpp`, `tests/declarations/declarators/array-4gib-ilp32.cpp`

## Context

On x86-64, a declaration of `char huge[1ull << 62];` (4 EiB) is rejected by
Clang 22 and 18 ("array is too large") and accepted by GCC 16 and 14 and EDG
7.0 when compiling. With `-m32`, Clang and GCC reject a 4 GiB array, GCC
because the bound doesn't fit the 32-bit size type.

## Evidence

`tests/RESULTS.md`, rows `array-too-large.cpp`, `array-4gib-lp64.cpp`, and
`array-4gib-ilp32.cpp`.

## Options

1. Leave the limit to each implementation (`impl-defined`), with only the
   requirement that the size fits the type of `sizeof`.
2. Require a minimum that every implementation must accept, so programs
   below it are portable.

## Provisional answer

Option 1. Implementations disagree on the limit, and the limit changes with
the target.

## Opinions
