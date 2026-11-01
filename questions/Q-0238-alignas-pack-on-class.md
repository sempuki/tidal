# Q-0238: Does `alignas(T...)` on a class take the strictest alignment of the elements?

**Status:** compiler defect
**Affects:** PACK-16, `tests/templates/packs/alignment-specifier-class.cpp`

## Context

`template <class... T> struct alignas(T...) S { char c; };` and then
`alignof(S<Eight, Two>)`, where `Eight` has alignment 8 and `Two` has
alignment 2.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | 8, the strictest element |
| EDG 7.0 | 8, the strictest element |
| GCC 14, 16, host | 2, the last element |

On a data member, `alignas(T...)` gives 8 on every compiler
(`alignment-specifier.cpp`).

## Evidence

The two tests above. A run of the class form on GCC 16 gives 2 for
`S<char, double, short>` and 1 for `S<double, char>`.

## Options

1. The strictest element, on a class as on a member (Clang, EDG, and
   GCC on a member).
2. Each place chooses, so a class can take the last element.

## Provisional answer

Option 1, and GCC's result on a class is a defect. Every compiler takes
the strictest element on a member, and several `alignas` on one
declaration also give the strictest alignment (ALN-1).

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
