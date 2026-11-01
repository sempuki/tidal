# Q-0108: Must carries_dependency be on the first declaration of a function?

**Status:** provisional
**Affects:** ATTR-2, `tests/declarations/attributes/carries-dependency-redeclared.cpp`

## Context

A function is declared without `carries_dependency` and redeclared with it. Clang 18 and 22 and EDG 7.0 reject the redeclaration, as for `noreturn`. GCC 14 and 16 accept the redeclaration, because GCC ignores the attribute.

## Evidence

`tests/declarations/attributes/carries-dependency-redeclared.cpp`, with
an override for GCC.

## Options

1. Not valid, as for noreturn.

2. Valid; the attribute has no effect.

## Provisional answer

Yes, as for noreturn (option 1). Clang and EDG agree, and the rule matches noreturn.

## Opinions
