# Q-0031: Is forming a pointer outside an array undefined, even without a read through it?

**Status:** settled
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
- an intermediate step that leaves the array, as in `a + 'a' - 'a'`,
  although the final position is inside the array
  (`tests/expressions/additive/intermediate-step-beyond-array.cpp`).
- a pointer that `++` moves two past the end
  (`tests/expressions/unary/increment-pointer-past-end-constant.cpp`).

## Evidence

The tests in Affects, and probes of the cases above on every compiler
in the matrix. GCC 16 behaves as GCC 14, so no newer version settles
the split.

## Options

1. Forming the pointer is undefined. GCC's acceptance is a missing
   diagnostic during constant evaluation.
2. Only a read through the pointer is undefined.

## Answer

Undefined (option 1). Two of three implementations diagnose the arithmetic itself.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) supports this answer with no doubt, and ADD-P6 gives the same answer: the valid positions run from the first element to one past the last, and outside those positions the behavior is undefined. Tidal records GCC 16's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for GCC 16.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. Adding an integer to a pointer into an array is defined only when the result is an element or the position one past the last element. Any other result is undefined at the addition, with no read needed. Constant evaluation rejects undefined operations, so a diagnostic is required there, and GCC's acceptance is a defect.
