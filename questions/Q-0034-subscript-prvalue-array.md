# Q-0034: Is a subscript of an array member of a prvalue an xvalue?

**Status:** provisional
**Affects:** PFX-3, `tests/expressions/postfix/subscript-prvalue-array.cpp`

## Context

`make().v[0]`, where `make()` returns a class with an array member `v`,
subscripts an array that belongs to a temporary object. Clang 22 and 18 and
GCC 16 and 14 give `decltype((make().v[0]))` as `int&&`, an xvalue. EDG 7.0
gives `int&`, an lvalue.

## Options

1. An xvalue, matching a member of a prvalue (VC-3). EDG's result is a
   possible defect.
2. An lvalue.

## Provisional answer

Option 1. Clang and GCC agree, and option 1 is consistent with VC-3.

## Opinions
