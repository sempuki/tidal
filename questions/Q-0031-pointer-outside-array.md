# Q-0031: Is forming a pointer outside an array undefined, even without a read through it?

**Status:** provisional
**Affects:** ADD-10, PFX-18, `tests/expressions/additive/pointer-beyond-array.cpp`, `tests/expressions/additive/pointer-before-array.cpp`, `tests/expressions/postfix/postfix-pointer-past-end-constant.cpp`

## Context

For `int a[3]`, the pointers `a + 4` (beyond one past the end) and `a - 1`
(before the start) are formed but never read through. During constant
evaluation, Clang 22 and 18 and EDG 7.0 reject both. GCC 16 and 14 accept
forming them, and reject only a read through them
(`deref-one-past-end.cpp` shows the read is rejected everywhere).

## Options

1. Forming the pointer is undefined. GCC's acceptance is a missing
   diagnostic during constant evaluation.
2. Only a read through the pointer is undefined.

## Provisional answer

Option 1. Two of three implementations diagnose the arithmetic itself.

## Opinions
