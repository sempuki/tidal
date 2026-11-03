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

Yes; Clang and EDG agree, GCC differs (option 1). Two of three implementations and the inventory agree. The
gap review supports option 2, and a contributor can argue for it with
that review.

The question stays open under decision 0051 (2026-11-03). Clang 22 and EDG 7.0 ask for the keyword after `typename` and in a base list, and GCC 16 does not, so two families disagree. Probes of 2026-11-03 give the same split for an alias target `typename T::box<int>` and for a function parameter of that type. Every compiler rejects the friend declaration `friend int T::get<int>(int);`, where `get` is a static member function template. DEP-P5 supports the provisional answer. The program states that a dependent name is a template, and `typename` states only that the name is a type.

Kept on 2026-11-03. The source-reader gap review of question cluster 6 (`reports/review/2026-11-03-gap-review-question-cluster-6.md`) reads the reference as supporting option 2. In that reading, a `<` after the last name in a type-only position starts an argument list without the keyword. These positions include the name after `typename`, a base list, and a parameter type. The same reading makes `T::box<int>` valid as an alias target without `typename`, and every compiler rejects that form. Clang 22 and EDG 7.0 follow option 1. GCC 16 follows option 2, except for the alias target without `typename`. DEP-P5 keeps option 1. The program states that a dependent name is a template, and `typename` or a type-only position states only that the whole name is a type. Option 2 would have to keep the alias target without `typename` not valid to match any compiler, and no principle of Tidal separates that position from the others. The alias target without `typename` is on the maintainer's list in `documentation/status.md`, because every compiler rejects it and the review reads it as valid.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 6 (`reports/review/2026-11-03-gap-review-question-cluster-6.md`) reads the reference as supporting option 2, and as going further than GCC. A `<` after the last name in a position where only a type can appear starts an argument list without the keyword, unless the name is part of a qualifier. These positions include the name after `typename`, a base, the target of an alias declaration, and parameter types. So the alias target without `typename` is valid too, which every compiler rejects. The friend declarator falls under the case of a declared name, and the review finds the friend form unclear.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
