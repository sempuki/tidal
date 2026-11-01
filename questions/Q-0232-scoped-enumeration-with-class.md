# Q-0232: Are the enumerators of a scoped member enumeration instantiated with the class?

**Status:** provisional
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

## Provisional answer

No; Clang and EDG agree, GCC differs (option 1). Two of three implementations and the inventory agree.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
