# Q-0248: Does an unqualified conversion function name find a conversion function of a dependent base?

**Status:** provisional
**Affects:** DEP-11, DEP-12, `tests/templates/dependent-names/dependent-base-conversion-function.cpp`, `tests/templates/dependent-names/dependent-base-conversion-function-independent.cpp`

## Context

`Derived<T>` derives from `Base<T>`, which declares `operator T()`. A
member function of `Derived<T>` calls `operator T()` without
`this->`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts, and calls `Base<T>::operator T` |
| GCC 14, 16, host | rejects: no argument depends on a template parameter |
| EDG 7.0 | rejects: the identifier is undefined |

Every compiler rejects `operator int()` without `this->` when the
target type is not dependent.

## Evidence

The test above. Item 17 of the gap review `reports/review/2026-10-31-gap-review-dependent-names-and-instantiation.md` says
that a conversion function name with a dependent target type is
looked up at the instantiation and finds the base's function.

## Options

1. The name is not found, as for any other name of a dependent base (GCC, EDG).
2. A conversion function name with a dependent target type is a dependent name, and lookup at the instantiation finds the base's function (Clang).

## Provisional answer

Yes; Clang agrees, GCC and EDG differ (option 2). The name `operator T` is a dependent name when `T` is dependent, and the lookup of a dependent name waits for the arguments (DEP-P1), when the base is known. DEP-11 states the answer. A name whose type is not dependent stays unfound (DEP-12).

Changed on 2026-11-03 to option 2. The review reads option 2, Clang 22 follows it, and DEP-P1 gives the same answer: the lookup of a dependent name waits for the arguments, when the base is known. DEP-11 now states option 2, and the table of DEP-P3 has a row for it. DEP-12 keeps the name whose type is not dependent. `dependent-base-conversion-function.cpp` expects the program to run, with overrides for GCC and EDG, and the new test `dependent-base-conversion-function-independent.cpp` checks `operator int`, which every compiler rejects.

Kept open on 2026-11-03 under decision 0054, because two newest families differ from the answer (GCC 16 and EDG 7.0). DEP-11 also changed on the same day, and a question settles only after a reader or a review has used the changed rule (decision 0055).

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) reads option 2. An unqualified conversion function name whose target type is dependent is a dependent name. It is looked up at the point of instantiation, so lookup can find a member of a dependent base. An example calls such a conversion function of a base given as a template argument without `this->` and accepts the call. GCC and EDG apply the older rule that never looks into a dependent base for an unqualified name. A name with a target type that is not dependent stays unfound, as every compiler finds.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
