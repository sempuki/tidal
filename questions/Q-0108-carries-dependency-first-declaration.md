# Q-0108: Must carries_dependency be on the first declaration of a function?

**Status:** provisional
**Affects:** ATTR-2, `tests/declarations/attributes/carries-dependency-redeclared.cpp`

## Context

A function is declared without `carries_dependency` and redeclared with it. Clang and EDG reject the redeclaration, as for `noreturn`. GCC accepts the redeclaration, because GCC ignores the attribute.

## Options

1. Not valid, as for noreturn.

2. Valid; the attribute has no effect.

## Provisional answer

Option 1. Clang and EDG agree, and the rule matches noreturn.

## Opinions
