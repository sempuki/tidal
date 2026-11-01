# Q-0104: Do a class object and its first member compare equal through void pointers?

**Status:** provisional
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

## Provisional answer

Option 1. Clang and GCC agree, and CLS-10 gives the two the same address.

## Opinions
