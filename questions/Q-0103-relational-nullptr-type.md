# Q-0103: Can two values of the type of nullptr be compared with a relational operator?

**Status:** provisional
**Affects:** CMP-9, `tests/expressions/comparison/relational-nullptr-values.cpp`

## Context

`a < b` where both operands have the type of `nullptr`. Clang and GCC reject the comparison. EDG 7.0 accepts it.

## Options

1. The comparison is not valid, as for two pointers to members.

2. The comparison is valid.

## Provisional answer

Option 1. Clang and GCC agree, and only `==` and `!=` apply to the type of `nullptr`.

## Opinions
