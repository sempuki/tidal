# Q-0248: Does an unqualified conversion function name find a conversion function of a dependent base?

**Status:** provisional
**Affects:** DEP-12, `tests/templates/dependent-names/dependent-base-conversion-function.cpp`

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

Option 1. Two of three implementations agree. The gap review supports option 2, and a contributor can argue for it with that review.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
