# Q-0226: Can the template keyword come before a member that has no argument list after it?

**Status:** provisional
**Affects:** DEP-26, `tests/templates/dependent-names/template-keyword-no-list.cpp`

## Context

`object.template field` names a data member with the keyword
`template` and no argument list. The object has a dependent type.

| Compiler | Result |
|---|---|
| GCC 14, 16, host | accepts |
| Clang 18, 22, host | rejects: an argument list is expected after the keyword |
| EDG 7.0 | rejects: an argument list for a template is expected |

## Evidence

The test above and the grid `template-keyword` (row "data member that
is not a template"). Inventory item F10 allows the keyword without a
list only before a class template or alias template.

## Options

1. The keyword without a list is valid only before a template that is
   a template template argument, or a class or alias template (Clang,
   EDG).
2. The keyword is ignored before a member that is not a template
   (GCC).

## Provisional answer

Option 1. Two of three implementations and the inventory agree.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
