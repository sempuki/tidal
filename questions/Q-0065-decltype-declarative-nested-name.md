# Q-0065: Can decltype name the class in a member definition outside the class?

**Status:** provisional
**Affects:** DTY-3, `tests/expressions/decltype/declarative-nested-name.cpp`

## Context

A member function can be defined outside its class with a qualified name.
A program can write the class part of that name with `decltype`, as in
`void decltype(s)::f() {}`. Clang and EDG reject it. GCC accepts it.

## Options

1. `decltype` must not occur in the qualified name of a declaration.
2. It is valid there.

## Provisional answer

Option 1. Clang and EDG agree. `decltype` stays valid in a qualified name
in an expression.

## Opinions
