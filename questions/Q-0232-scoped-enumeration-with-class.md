# Q-0232: Are the enumerators of a scoped member enumeration instantiated with the class?

**Status:** settled
**Affects:** INST-5, `tests/templates/instantiation/class-scoped-enumeration-unused.cpp`

## Context

`enum class Kind { first = T::nothing };` in a class template, with
`Box<int>` defined as an object and `Kind` not used. An unscoped
enumeration with the same enumerator is rejected by every compiler.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts |
| EDG 7.0 | accepts |
| GCC 14, 16, host | rejects |

## Evidence

The test above and `class-unscoped-enumeration.cpp`. Inventory item H2
lists scoped enumerations with the members whose declarations only
are made with the class, and item H3 lists unscoped enumerations with
the definitions that are made.

## Options

1. A scoped enumeration's definition waits for a use (Clang, EDG).
2. The definition is made with the class (GCC).

## Answer

No; Clang and EDG agree, GCC differs (option 1). Two of three implementations and the inventory agree.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) supports this answer with no doubt, and INST-P2 gives the same answer: a scoped enumeration is made with the class as a declaration only. GCC 16 still differs. Tidal records GCC 16's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for GCC 16.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) agrees with option 1. The instantiation of a class specialization makes only the declaration of a scoped member enumeration, and the definition of an unscoped one. So the enumerators of the scoped enumeration are not instantiated with the class. GCC's rejection is a defect.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
