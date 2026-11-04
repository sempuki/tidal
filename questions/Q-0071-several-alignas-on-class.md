# Q-0071: Which alignas applies when a class has several?

**Status:** settled
**Affects:** ALN-1, `tests/declarations/alignment/strictest-wins.cpp`

## Context

A class definition has several `alignas` specifiers with different values. Clang and EDG use the strictest. GCC 14, 16, and the host GCC 16.2 use the last one.

On a variable and on a non-static data member, GCC uses the strictest
of several `alignas`, as Clang and EDG do. `alignas(16) alignas(8)` on
either gives an alignment of 16 with every compiler.

## Evidence

`tests/declarations/alignment/strictest-wins.cpp`, with an override for
GCC.

## Options

1. The strictest alignment applies.
2. The last alignment applies.

## Answer

The strictest alignment (option 1). Clang and EDG agree, and every compiler, GCC included, uses
the strictest on variables and data members.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) supports this answer with no doubt, and ALN-P1 gives the same answer: every alignment is a power of two, and the strictest requirement wins. GCC 16 still differs. Tidal records GCC 16's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for GCC 16.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. The strictest of all the alignment specifiers on an entity applies, with no exception for classes. GCC's result on a class is a defect.
