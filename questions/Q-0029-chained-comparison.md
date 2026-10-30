# Q-0029: Is a chained comparison such as `3 > 2 > 1` valid?

**Status:** provisional
**Affects:** CMP-1 and the chained-comparison warning rule, `tests/expressions/comparison/chained-comparison.cpp`, `tests/expressions/comparison/chained-comparison-warns.cpp`

## Context

`3 > 2 > 1` groups as `(3 > 2) > 1`, which compares a `bool` with `1`.
Clang 22 rejects it by default, through a `-Wparentheses` diagnostic that
is an error unless disabled. Clang 18, GCC 16 and 14, and EDG 7.0 accept
it. Clang 18 also gives no warning for a chained comparison of variables.

## Options

1. Valid. Rejecting it by default is an implementation's diagnostic
   choice, and a warning is recommended practice.
2. An implementation may reject it.

## Provisional answer

Option 1. The grouping and result are well defined, and four of five
compiler versions accept it.

## Opinions
