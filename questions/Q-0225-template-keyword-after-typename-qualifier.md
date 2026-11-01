# Q-0225: Does a name after typename T:: need the template keyword before an argument list?

**Status:** provisional
**Affects:** DEP-24, `tests/templates/dependent-names/template-keyword-needed-after-typename.cpp`

## Context

`typename T::box<int>` names a member class template of `T` with an
argument list and no `template` keyword. The same holds for an alias
target `typename T::pointer<int>`.

| Compiler | Result |
|---|---|
| GCC 14, 16, host | accepts |
| Clang 18, 22, host | rejects: use the `template` keyword |
| EDG 7.0 | rejects |

## Evidence

The test above and the grid `template-keyword` (rows "after typename
T::" and "alias target"). Clang's diagnostic list (clang-diag-sema)
has a message for a missing `template` before a dependent template
name. Inventory items F5 and F13 say that the keyword is needed after a
dependent qualifier and can appear after `typename`.

## Options

1. The keyword is needed (Clang, EDG).
2. After `typename`, a `<` after the name starts an argument list
   without the keyword (GCC).

## Provisional answer

Option 1. Two of three implementations and the inventory agree.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
