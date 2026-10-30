# Q-0030: May a pointer be compared with `<` or `<=>` to the literal 0?

**Status:** provisional
**Affects:** CMP-9, CMP-13, `tests/expressions/comparison/pointer-relational-zero.cpp`, `tests/expressions/comparison/three-way-pointer-zero.cpp`

## Context

`p < 0` and `p <=> 0`, where `p` is a pointer, are rejected by Clang 22 and
18 and GCC 16 and 14. EDG 7.0 accepts both. Equality comparison with `0`
(`p == 0`) is accepted everywhere.

## Options

1. Rejected. Ordered comparisons need two pointers, and EDG's acceptance
   is a possible defect or extension.
2. Valid, with `0` converted to a null pointer as for equality.

## Provisional answer

Option 1. Clang and GCC, in both versions, reject it.

## Opinions
