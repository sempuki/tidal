# Q-0049: Is arithmetic on a base class pointer into an array of a derived class undefined?

**Status:** provisional
**Affects:** ADD-14, `tests/expressions/additive/base-pointer-derived-array.cpp`

## Context

A `Base*` that points into an array of `Derived`, where `Derived` adds members, moves by the wrong element size when incremented. During constant evaluation, Clang and EDG reject the arithmetic. GCC accepts it.

## Options

1. Undefined: the element type of the pointer must match the array's element type. GCC is missing a diagnostic.
2. Only a read through the pointer is undefined.

## Provisional answer

Option 1. Two of three implementations reject the arithmetic.

## Opinions
