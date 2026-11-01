# Q-0241: Must an implementation diagnose a deduction guide whose template parameter cannot be deduced?

**Status:** provisional
**Affects:** CTAD-13, `tests/templates/class-template-deduction/guide-not-deducible.cpp`

## Context

`template <class T, class U> Box(T) -> Box<U>;` declares a guide in
which no parameter gives `U` a value, and `U` has no default.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects the declaration: a template parameter cannot be deduced |
| GCC 14, 16, host | accepts |
| EDG 7.0 | accepts |

Such a guide is never viable, so the guide changes no deduction.

## Evidence

The test above, and the inventory item H4.

## Options

1. The guide is not valid, and an implementation need not diagnose it
   (`no-diagnostic`).
2. The guide is not valid and must be rejected (`reject`).
3. The guide is valid and is never chosen.

## Provisional answer

Option 1. The inventory says that the guide is not valid, and two of
three implementations accept the declaration. A rule that needs no
diagnostic allows both results.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
