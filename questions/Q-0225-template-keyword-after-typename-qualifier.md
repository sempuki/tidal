# Q-0225: Does a name after typename T:: need the template keyword before an argument list?

**Status:** provisional
**Affects:** DEP-24, `tests/templates/dependent-names/template-keyword-needed-after-typename.cpp`, `tests/templates/dependent-names/template-keyword-needed-base.cpp`

## Context

`typename T::box<int>` names a member class template of `T` with an
argument list and no `template` keyword. The same holds for an alias
target `typename T::pointer<int>` and for a base `T::box<int>`.

| Compiler | after `typename` | base | alias target without `typename` | friend declarator |
|---|---|---|---|---|
| GCC 14, 16, host | accepts | accepts | rejects | rejects |
| Clang 18, 22, host | rejects: use the `template` keyword | rejects | rejects | rejects |
| EDG 7.0 | rejects | rejects | rejects | rejects |

## Evidence

The tests above, `template-keyword-needed-alias-target.cpp`, and the
grid `template-keyword` (rows "after typename T::" and "alias target").
Item 1 of the gap review
`reports/review/2026-10-31-gap-review-dependent-names-and-instantiation.md`
says that a `<` after the last name in such a position starts an
argument list, as GCC reads it. GCC does not read `T::box<int>` that
way as an alias target without `typename`, so no compiler follows the
review in every position. Clang's diagnostic list (clang-diag-sema)
has a message for a missing `template` before a dependent template
name. Inventory items F5 and F13 say that the keyword is needed after a
dependent qualifier and can appear after `typename`.

## Options

1. The keyword is needed (Clang, EDG).
2. After `typename`, a `<` after the name starts an argument list
   without the keyword (GCC).

## Provisional answer

Option 1. Two of three implementations and the inventory agree. The
gap review supports option 2, and a contributor can argue for it with
that review.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
