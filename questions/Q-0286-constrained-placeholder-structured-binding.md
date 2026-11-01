# Q-0286: Can a structured binding declaration have a constrained placeholder?

**Status:** provisional
**Affects:** CPT-37, `tests/templates/concepts/placeholder-structured-binding.cpp`

## Context

A function declares `Any auto [a, b] = Pair{1, 2};`, where the concept
`Any` is always satisfied.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects: cannot be declared with constrained 'auto' |
| GCC 14, 16, host | rejects: structured binding declaration cannot have constrained 'auto' type |
| EDG 7.0 | accepts |

## Evidence

The test above. GCC gives its error as a pedantic diagnostic. Every
compiler rejects a constrained placeholder in a functional cast
(`placeholder-functional-cast.cpp`).

## Options

1. Not valid (Clang, GCC). A structured binding declaration takes only
   the plain placeholder.
2. Valid, with the constraint checked on the deduced type (EDG).

## Provisional answer

No; Clang and GCC agree, EDG differs (option 1). Two of three implementations agree, and the gap review of
concepts and constraints (item 17) describes the same rule.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
