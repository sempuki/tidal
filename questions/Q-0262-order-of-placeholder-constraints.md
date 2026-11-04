# Q-0262: Is the requires clause after the template parameter list checked before the constraints of placeholder parameters?

**Status:** settled
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

## Answer

Yes; GCC and EDG agree, Clang differs (option 1). Two of three implementations agree, and the order follows the order in which the declaration writes the parts.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) supports this answer with no doubt, and CNS-P1 gives the same answer: step 1 collects the clause after the head before the placeholder constraints, and CNS-P2 stops a conjunction at a false operand. Clang 22 still differs. Tidal records Clang 22's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for Clang 22.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) agrees with option 1. The associated constraints form one conjunction in a fixed order: the type constraints of the parameter list, the clause after the list, the type constraints of placeholder parameters, and the trailing clause. That order is the order of checking, and a conjunction stops at the first false operand. Clang's order is a defect.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
