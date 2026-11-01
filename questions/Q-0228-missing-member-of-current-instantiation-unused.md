# Q-0228: Is a missing member of the current instantiation diagnosed in a template that is not used?

**Status:** provisional
**Affects:** DEP-17, DEP-30, `tests/templates/dependent-names/current-member-missing.cpp`, `tests/templates/dependent-names/current-member-missing-used.cpp`, `tests/templates/dependent-names/this-member-missing.cpp`, `tests/templates/dependent-names/this-member-missing-specialized.cpp`

## Context

In a class template with no dependent base, `Box::missing` names a
member that the class does not declare. The template is never
instantiated.

| Compiler | Result |
|---|---|
| GCC 14 | accepts |
| GCC 16, host | rejects, in the warning group `-Wtemplate-body` |
| Clang 18, 22, host | rejects |
| EDG 7.0 | rejects |

## Evidence

The test above and the grid `current-instantiation` (column "missing
member, no dependent base"). The sanitized Clang lookup source treats a
name that the current instantiation does not declare as dependent only
when the class has a dependent base. Inventory item C8 says the name is an error at the
definition. Item 4 of the gap review
`reports/review/2026-10-31-gap-review-dependent-names-and-instantiation.md`
says that the implementation can report the error before an
instantiation and need not. GCC 16 accepts the template under
`-Wno-template-body`, the flag that also turns off the independent
errors of DEP-30. Every compiler rejects the name when the class is
instantiated (`current-member-missing-used.cpp`).

## Options

1. The implementation must reject the unused template, and GCC 14 has
   a defect that GCC 16 fixes.
2. The name is an independent error, and DEP-30 needs no diagnostic
   while no specialization is made.

## Provisional answer

No diagnostic required while unused; GCC 14 accepts, others reject (option 2). The name is not dependent, so DEP-30 covers the unused
template, and every result lies within that rule. DEP-17 stays a
`reject` rule for a class that is instantiated, where every compiler
rejects.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
