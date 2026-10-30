# Q-0028: Is converting a floating value above the target's finite range implementation-defined or undefined?

**Status:** provisional
**Affects:** CONV-11, `tests/expressions/conversions/floating-narrowing-out-of-range.cpp`

## Context

`constexpr float f = 1e300;` converts a `double` that is larger than the
largest finite `float`. Clang 22 and 18 and GCC 16 and 14 accept it during
constant evaluation and give positive infinity. EDG 7.0 rejects it
("floating-point value does not fit in required floating-point type").

On IEEE 754 targets, infinity is a value of `float`, so 1e300 lies between
the largest finite `float` and infinity. A value between two values of the
target is a rounding case. A value outside every value of the target is
out of range.

## Options

1. `impl-defined`: on targets where the floating type has infinities, the
   result is one of the two neighboring values. Clang and GCC conform; EDG
   is stricter than necessary.
2. `undefined`: any value above the largest finite value is out of range.
   EDG conforms, and Clang and GCC accept an extension.

## Provisional answer

Option 1. It matches the run-time behavior of every compiler on IEEE
targets, and two of three implementations during constant evaluation.

## Opinions
