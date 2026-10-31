# Q-0036: May the size in an array `new` have floating-point type?

**Status:** provisional
**Affects:** NEW-2, `tests/expressions/new-delete/array-non-integral-size.cpp`

## Context

`new int[1.5]` is accepted by Clang 22 and 18, with a `-Wliteral-conversion` warning, and by EDG 7.0. Both convert the size to 1. GCC 16 and 14 reject it ("must have integral or enumeration type").

## Options

1. Accepted: the size converts to the type of `sizeof`, as any value converts.
2. Rejected: only integer and enumeration sizes are allowed.

## Provisional answer

Option 1. Two of three implementations accept the floating-point size.

## Opinions
