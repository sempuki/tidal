# Q-0051: Is comparing a one-past-the-end pointer with a pointer to the next object rejected during constant evaluation?

**Status:** provisional
**Affects:** CMP-20, `tests/expressions/comparison/one-past-end-adjacent-constant.cpp`

## Context

Whether a pointer one past the end of an object equals a pointer to an
object that follows it in memory depends on layout, so the result is
unspecified. During constant evaluation, Clang 22 and 18 and GCC 16 and
14 reject the comparison. EDG 7.0 accepts it.

Inside one array of arrays, every implementation accepts the same kind
of comparison during constant evaluation. The pointer one past the end
of one row equals the pointer to the first element of the next row
(CMP-26). The rejection applies only when the two pointers lie in
different complete objects, whose layout the implementation chooses.

## Evidence

The test in Affects, `tests/expressions/comparison/row-end-equals-next-row.cpp`,
and a new probe on every compiler in the matrix. EDG 7.0 is the only EDG
version in the matrix.

## Options

1. Rejected during constant evaluation; unspecified at run time.
2. Accepted during constant evaluation, with an unspecified value.

## Provisional answer

Option 1. Clang and GCC agree, and option 1 matches Q-0033.

## Opinions
