# Q-0403: In constant evaluation, does a defaulted `==` compare every element of a member that is an array of arrays?

**Status:** provisional
**Affects:** DCMP-1, `tests/classes/comparisons/eq-array-of-arrays-member-constant.cpp`, `tests/classes/comparisons/eq-array-of-arrays-member.cpp`
**Source:** finding W19-1 of `reports/clean-room/W19/report.md`, from the probes that a clean-room writer ran for it.
**Waits for:** a first review; newest families that agree

## Context

A class has a member `int c[2][2]` and a defaulted `==`. Two constant
objects differ only in the second row. At run time every compiler gives
`false` for `a == d` (`eq-array-of-arrays-member.cpp`). In constant
evaluation the compilers differ.

| Implementation | `a == b` (equal) | `a == d` (second row differs) |
|---|---|---|
| Clang 22 | true | false |
| GCC 16 | true | true |
| EDG 7.0 | not a constant | not a constant |

## Evidence

The tests in **Affects**. The constant test has overrides for GCC and
EDG.

## Options

1. The defaulted `==` compares every element, down to elements that are
   not arrays, also in constant evaluation (Clang, and every compiler at
   run time).
2. The result in constant evaluation follows each implementation.

## Provisional answer

Every element, also in constant evaluation (option 1). DCMP-1 gives one
meaning to the defaulted `==`, and every compiler gives that meaning at
run time.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
