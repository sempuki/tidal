# Q-0031: Is forming a pointer outside an array undefined, even without a read through it?

**Status:** provisional
**Affects:** ADD-10, PFX-18, `tests/expressions/additive/pointer-beyond-array.cpp`, `tests/expressions/additive/pointer-before-array.cpp`, `tests/expressions/postfix/postfix-pointer-past-end-constant.cpp`

## Context

For `int a[3]`, the pointers `a + 4` (beyond one past the end) and `a - 1`
(before the start) are formed but never read through. During constant
evaluation, Clang 22 and 18 and EDG 7.0 reject both. GCC 16 and 14 accept
forming them, and reject only a read through them
(`deref-one-past-end.cpp` shows the read is rejected everywhere).

GCC's acceptance covers these cases too, and Clang and EDG reject each
one:

- `a + 100` and `a - 1` in a `constexpr` initializer,
- `a + 4` inside a `constexpr` function, compared with `nullptr`,
- `a + 4` moved back by two and then read. GCC gives the value of
  `a[2]`.

## Evidence

The tests in Affects, and probes of the cases above on every compiler
in the matrix. GCC 16 behaves as GCC 14, so no newer version settles
the split.

## Options

1. Forming the pointer is undefined. GCC's acceptance is a missing
   diagnostic during constant evaluation.
2. Only a read through the pointer is undefined.

## Provisional answer

Option 1. Two of three implementations diagnose the arithmetic itself.

## Opinions
