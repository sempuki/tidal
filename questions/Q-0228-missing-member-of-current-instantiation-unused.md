# Q-0228: Is a missing member of the current instantiation diagnosed in a template that is not used?

**Status:** provisional
**Affects:** DEP-17, `tests/templates/dependent-names/current-member-missing.cpp`

## Context

In a class template with no dependent base, `Box::missing` names a
member that the class does not declare. The template is never
instantiated.

| Compiler | Result |
|---|---|
| GCC 14 | accepts |
| GCC 16, host | rejects |
| Clang 18, 22, host | rejects |
| EDG 7.0 | rejects |

## Evidence

The test above and the grid `current-instantiation` (column "missing
member, no dependent base"). The sanitized Clang lookup source treats a
name that the current instantiation does not declare as dependent only
when the class has a dependent base. Inventory item C8 says the name is an error at the
definition.

## Options

1. The program is not valid, and GCC 14 has a defect that GCC 16
   fixes.
2. The rule needs no diagnostic.

## Provisional answer

Option 1. Every other implementation, including the newer GCC,
rejects the template.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
