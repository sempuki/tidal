# Q-0263: Is an atomic constraint whose type is never bool an error when a short circuit skips it?

**Status:** provisional
**Affects:** CNS-9, CNS-39, `tests/templates/constraints/atomic-not-bool-skipped.cpp`, `tests/templates/constraints/atomic-not-bool-nondependent-concept.cpp`

## Context

`concept Odd = (sizeof(T) > 100) && (sizeof(T));`. The right operand
has an integer type for every `T`. For `int`, the left operand is
false, so satisfaction never substitutes into the right operand.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects: atomic constraint must be of type bool |
| GCC 14, 16, host | accepts |
| EDG 7.0 | rejects: atomic constraint must have type bool |

## Evidence

The test above. Every compiler rejects the same operand when the check reaches it (`atomic-not-bool.cpp`).

With no use of the template at all, Clang and EDG reject the concept
(`atomic-not-bool-nondependent-concept.cpp`). GCC accepts the concept,
and rejects the same operand in the requires clause of a function
template at its definition (`atomic-not-bool-nondependent-clause.cpp`).
CNS-39 states the check at the definition.

## Options

1. Not valid (Clang, EDG). The type does not depend on the
   arguments, so the constraint can never be satisfied.
2. Valid until the check reaches the operand (GCC).

## Provisional answer

Yes; Clang and EDG agree, GCC differs (option 1). Two of three implementations agree. A constraint that is not valid for any argument is not valid in the template, as DEP-30 treats other constructs.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
