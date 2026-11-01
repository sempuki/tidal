# Q-0106: Can a parameter have a reference to a function type with a ref-qualifier?

**Status:** provisional
**Affects:** DCL-16, `tests/declarations/declarators/qualified-function-type-parameter.cpp`

## Context

A type alias names a function type with a ref-qualifier. A parameter is declared as a reference to that type. Clang and GCC reject the declaration. EDG 7.0 accepts it.

## Options

1. Not valid, as for a pointer to such a type.

2. Valid.

## Provisional answer

No, as for a pointer to such a type (option 1). Clang and GCC agree, and a pointer to the same type is rejected by all three.

## Opinions
