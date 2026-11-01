# Q-0282: Is the constraint of a constrained constant parameter checked in the order of the template parameters?

**Status:** provisional
**Affects:** CNS-1, `tests/templates/constraints/order-constant-placeholder-after-type.cpp`

## Context

A function template has the head `<Trapped T, Wide auto N>`. Checking
`Trapped<int>` instantiates a class whose static assertion fails. The
call gives `N` the value `'a'`, which fails `Wide`. A second template
without constraints can take the call.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects: the static assertion fails |
| GCC 16, host | rejects: the static assertion fails |
| GCC 14 | accepts: chooses the second template |
| EDG 7.0 | accepts: chooses the second template |

## Evidence

The test above. When the constant parameter comes first, every
compiler chooses the second template and never instantiates the class
(`order-constant-placeholder.cpp`). Every compiler checks the
constraint of `N` before a requires clause after the head
(`order-constant-placeholder-before-clause.cpp`).

## Options

1. The constraints of the head are checked in parameter order, type
   and constant parameters together (Clang, GCC 16).
2. The constraint of a constant placeholder is checked first, before
   the type constraints of the head (GCC 14, EDG).

## Provisional answer

Yes; Clang and GCC 16 agree, GCC 14 and EDG differ (option 1). The newest Clang and GCC agree, and the order follows the
written order of the head, as CNS-1 does for the other parts.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
