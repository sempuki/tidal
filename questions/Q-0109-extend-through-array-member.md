# Q-0109: Does a reference bound to an element of an array member of a temporary extend the temporary?

**Status:** provisional
**Affects:** REF-4, `tests/declarations/initialization/references/extend-through-array-member-subscript.cpp`

## Context

An rvalue reference binds to one element of an array member of a class prvalue. Clang and GCC accept the binding and keep the temporary alive with the reference. EDG 7.0 rejects the binding as an rvalue reference to an lvalue.

## Options

1. The binding is valid and extends the temporary.

2. The binding is not valid.

## Provisional answer

Option 1. Clang and GCC agree, and REF-4 already covers a member of the temporary.

## Opinions
