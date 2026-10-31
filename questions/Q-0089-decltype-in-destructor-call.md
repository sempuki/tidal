# Q-0089: Can decltype name the class in an explicit destructor call?

**Status:** provisional
**Affects:** DTY-3, `tests/expressions/decltype/destructor-call.cpp`

## Context

A program calls a destructor explicitly and writes the class
name as `decltype` of the object, as in `t.~decltype(t)()`. Clang and EDG
accept the call. GCC rejects it and expects a class name after `~`.

## Options

1. `decltype` can name the class in an explicit destructor call.
2. Only a class name or a type alias can follow `~`.

## Provisional answer

Option 1. Clang and EDG agree, and DTY-3 lets
`decltype` occur where a type name can occur.

## Opinions
