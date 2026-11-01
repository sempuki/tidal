# Q-0103: Can two values of the type of nullptr be compared with a relational operator?

**Status:** provisional
**Affects:** CMP-9, `tests/expressions/comparison/relational-nullptr-values.cpp`

## Context

`a < b` where both operands have the type of `nullptr`. Clang 22 and 18
and GCC 16 and 14 reject the comparison. EDG 7.0 accepts `<`, `<=`, and
`>=`, and during constant evaluation gives `false`, `true`, and `true`.
EDG rejects `<=>` on the same operands
(`three-way-nullptr-values.cpp`), so EDG orders the two values with
one operator and not with the other.

## Evidence

The test in Affects, `tests/expressions/comparison/three-way-nullptr-values.cpp`,
and a probe of the three operators in a `static_assert` on every
compiler in the matrix. EDG 7.0 is the only EDG version in the matrix.

## Options

1. The comparison is not valid, as for two pointers to members.
2. The comparison is valid.

## Provisional answer

Option 1. Clang and GCC agree, and only `==` and `!=` apply to the type
of `nullptr`. EDG's rejection of `<=>` agrees with option 1.

## Opinions
