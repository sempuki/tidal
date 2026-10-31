# Q-0099: Is a subscript of an array xvalue an xvalue?

**Status:** provisional
**Affects:** VC-3, `tests/expressions/value-categories/xvalue-array-subscript.cpp`

## Context

An array xvalue (a cast to an rvalue reference to an array) or an array prvalue is subscripted. Clang and GCC give an xvalue. EDG 7.0 gives an lvalue.

## Options

1. The result is an xvalue.

2. The result is an lvalue.

## Provisional answer

Option 1. Clang and GCC agree, and it matches member access on an xvalue.

## Opinions
