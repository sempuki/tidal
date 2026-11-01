# Q-0261: Are the constraints of a function template checked before its return type is formed?

**Status:** provisional
**Affects:** CNS-11, `tests/templates/constraints/before-function-type.cpp`

## Context

The return type instantiates a class whose static assertion fails.
The constraint is not satisfied, and another function takes the call.

| Compiler | Result |
|---|---|
| Clang 18 | rejects: the static assertion fails |
| Clang 22, host | accepts |
| GCC 14, 16, host | accepts |
| EDG 7.0 | accepts |

## Evidence

The test above. Every compiler accepts the same case with the class in a parameter type (`before-parameters.cpp`).

## Options

1. The constraints come first, and a failure stops the substitution
   (Clang 22, GCC, EDG).
2. The return type comes first (Clang 18).

## Provisional answer

Yes; every current compiler agrees, Clang 18 differs (option 1). Every current implementation agrees.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
