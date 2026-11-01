# Q-0270: Can a concept definition have an attribute after the concept's name?

**Status:** provisional
**Affects:** CPT-3, `tests/templates/concepts/attribute.cpp`

## Context

| Compiler | Result |
|---|---|
| Clang 18 | rejects: an attribute list cannot appear here |
| Clang 22, host | accepts |
| GCC 14, 16, host | accepts |
| EDG 7.0 | accepts |

## Evidence

The test above.

## Options

1. Valid (every current implementation).
2. Not valid (Clang 18).

## Provisional answer

Yes; every current compiler agrees, Clang 18 differs (option 1). Every current implementation agrees.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
