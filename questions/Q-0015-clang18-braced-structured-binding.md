# Q-0015: Is `auto [a, b]{arr};` valid when `arr` is an array?

**Status:** provisional
**Affects:** SB-7, `tests/declarations/structured-bindings/initializer-braces.cpp`

## Context

A structured binding may be initialized with `=`, parentheses, or braces.
With an array initializer in braces, Clang 18.1.3 rejects the declaration
("cannot initialize an array element of type 'int' with an lvalue of type
'int[2]'"). Clang 22.1.8, GCC 16 and 14, and EDG 7.0 accept it, and all
compilers accept the `=` and parenthesized forms.

## Evidence

`tests/RESULTS.md`, rows `initializer-equals.cpp`,
`initializer-parens.cpp`, and `initializer-braces.cpp`.

## Options

1. Valid. Clang 18's rejection is a defect fixed in a later Clang.
2. Implementation-defined.

## Provisional answer

Option 1. The newer version of the same implementation and both other
implementations accept it.

## Opinions
