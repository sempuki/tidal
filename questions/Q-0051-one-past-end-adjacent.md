# Q-0051: Is comparing a one-past-the-end pointer with a pointer to the next object rejected during constant evaluation?

**Status:** provisional
**Affects:** CMP-20, `tests/expressions/comparison/one-past-end-adjacent-constant.cpp`

## Context

Whether a pointer one past the end of an object equals a pointer to an object that follows it in memory depends on layout, so the result is unspecified. During constant evaluation, Clang and GCC reject the comparison. EDG 7.0 accepts it.

## Options

1. Rejected during constant evaluation; unspecified at run time.
2. Accepted during constant evaluation, with an unspecified value.

## Provisional answer

Option 1. Clang and GCC agree, and option 1 matches Q-0033.

## Opinions
