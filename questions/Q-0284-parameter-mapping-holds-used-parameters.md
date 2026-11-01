# Q-0284: Does the mapping of an atomic constraint hold only the template parameters that its expression uses?

**Status:** provisional
**Affects:** CNS-4, CNS-7, `tests/templates/constraints/mapping-unused-parameter.cpp`, `tests/templates/constraints/mapping-unused-parameter-not-substituted.cpp`

## Context

The concept `First<T, U>` is `sizeof(T) > 1`, which does not use `U`.
Two cases depend on whether the mapping holds `U`:

- Identity. One function template requires `First<T, int>`, another
  `First<T, long> && true`. If the two concept-ids give one atomic
  constraint, the second template is more constrained.
- Substitution. The concept `Outer<T>` is `First<T, typename
  T::missing>`. If the argument for `U` is never substituted,
  `Outer<int>` is satisfied.

| Compiler | Identity | Substitution |
|---|---|---|
| Clang 22, host | second template chosen | satisfied |
| Clang 18 | second template chosen | not satisfied |
| GCC 14, 16, host | second template chosen | satisfied |
| EDG 7.0 | call ambiguous | not satisfied |

## Evidence

The two tests above. When two wrapper concepts pass the same computed
argument to one inner concept, every compiler treats the two atoms as
identical (`mapping-same-arguments.cpp`). The gap review of concepts
and constraints (item 10) describes the mapping as holding only the
parameters that occur in the expression.

## Options

1. The mapping holds only the parameters that the expression uses. An
   unused argument takes no part in identity and is not substituted
   (Clang 22, GCC).
2. The mapping holds every parameter of the concept (EDG).

## Provisional answer

Yes; Clang 22 and GCC agree, EDG and Clang 18 differ (option 1). The newest Clang and GCC agree on both cases, and the review
describes the same model.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
