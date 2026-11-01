# Q-0254: Can a variable template's definition follow its explicit instantiation definition?

**Status:** provisional
**Affects:** INST-20, `tests/templates/instantiation/explicit-variable-before-definition.cpp`

## Context

`template <class T> extern int value;`, then
`template int value<int>;`, then `template <class T> int value = 2;`,
in one translation unit.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts |
| EDG 7.0 | accepts |
| GCC 14, 16, host | rejects: no definition available |

Every compiler accepts the same order for a function template, and the
specialization is emitted
(`explicit-definition-before-template-definition.cpp`).

## Evidence

The tests above. Item 38 of the gap review `reports/review/2026-10-31-gap-review-dependent-names-and-instantiation.md` says
that the definition must be in the unit and need not come first.

## Options

1. The definition can follow the explicit instantiation (Clang, EDG).
2. The definition of a variable template must come first (GCC).

## Provisional answer

Yes; Clang and EDG agree, GCC differs (option 1). Two of three implementations and the gap review agree, and every compiler allows the order for a function.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
