# Q-0174: Is a partial specialization valid when it is not more specialized than the primary template?

**Status:** provisional
**Affects:** TSP-11

## Context

`template <class T, class U> struct Pair;` and the partial
specialization `template <class T, class U> struct Pair<U, T>`. The
argument list only swaps the parameters, so the partial specialization
matches every argument list.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects: not more specialized than the primary template |
| GCC 14, 16, host | accepts |
| EDG 7.0 | accepts |

All compilers reject a partial specialization whose argument list is
the primary template's parameter list in the same order.

## Evidence

`tests/templates/specialization/partial-reordered.cpp`, with overrides
for GCC and EDG.

## Options

1. The partial specialization must be more specialized than the primary
   template, and the implementation diagnoses it (Clang).
2. The declaration is valid (GCC, EDG).

## Provisional answer

Option 1. A partial specialization that matches every argument list
takes the place of the primary template, which TSP-11 already forbids
for the plain parameter list. The check needs partial ordering, which
roadmap item O specifies.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
