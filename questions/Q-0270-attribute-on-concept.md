# Q-0270: Can a concept definition have an attribute after the concept's name?

**Status:** settled
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

## Answer

Yes; every current compiler agrees, Clang 18 differs (option 1). Every current implementation agrees.

Settled on 2026-11-01 under decision 0051, because the newest version of each baseline family (Clang 22, GCC 16, EDG 7.0) agrees with this answer. Clang 18 still rejects the attribute, and the test keeps its override for Clang 18.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
