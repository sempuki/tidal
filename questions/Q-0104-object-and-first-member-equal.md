# Q-0104: Do a class object and its first member compare equal through void pointers?

**Status:** settled
**Affects:** CMP-15, `tests/expressions/comparison/struct-and-first-member-equal.cpp`

## Context

A pointer to a class object and a pointer to its first non-static data
member are converted to `const void*` and compared with `==`. Clang 22
and 18 and GCC 16 and 14 give `true` during constant evaluation. EDG 7.0
gives `false`, and gives `true` for `!=`. EDG gives the same results
for a local object inside a `constexpr` function. EDG compares a pointer
with itself as equal, and the object with its second member as not
equal, as the other compilers do.

At run time, Clang and GCC give `true` on every configuration. EDG has
no code generation, so the matrix has no run-time result for EDG.

## Evidence

The test in Affects, and probes of `==`, `!=`, and a local object on
every compiler in the matrix. EDG 7.0 is the only EDG version in the
matrix.

## Options

1. The pointers are equal, because they hold the same address.
2. The result is unspecified.

## Answer

Yes; the pointers hold the same address (option 1). Clang and GCC agree, and CLS-10 gives the two the same address.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) supports this answer with no doubt, and CMP-P3 gives the same answer: two pointers are equal when they hold one address, and an object and its first member can hold one address. EDG 7.0 still differs. Tidal records EDG 7.0's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for EDG 7.0.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. The class in the test is standard-layout, so the object and its first member are pointer-interconvertible and share one address. Two pointers with one address compare equal, and the result is specified, so constant evaluation must give `true`. EDG's result is a defect.
