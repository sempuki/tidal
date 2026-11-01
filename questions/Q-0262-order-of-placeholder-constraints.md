# Q-0262: Is the requires clause after the template parameter list checked before the constraints of placeholder parameters?

**Status:** provisional
**Affects:** CNS-1, `tests/templates/constraints/order-placeholder.cpp`

## Context

`template <class T> requires Big<T> int pick(T, Caught auto)`. For
`pick(1, 1)`, `Big<int>` is false, and checking `Caught<int>`
instantiates a class whose static assertion fails.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects: Clang checks the placeholder's constraint first |
| GCC 14, 16, host | accepts |
| EDG 7.0 | accepts |

## Evidence

The test above. Every compiler checks the type constraints of the
template head before the leading clause, and the placeholder
constraints before the trailing clause (`order.cpp`).

## Options

1. The leading clause comes before the placeholder constraints (GCC,
   EDG). The placeholder parameters come after the template head in
   the order of declaration (DED-34).
2. The placeholder constraints come with the other type constraints,
   before the leading clause (Clang).

## Provisional answer

Yes; GCC and EDG agree, Clang differs (option 1). Two of three implementations agree, and the order follows the order in which the declaration writes the parts.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
