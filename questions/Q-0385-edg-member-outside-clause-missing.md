# Q-0385: Must a member defined after the body of a class template repeat the requires clause of the class's template head?

**Status:** provisional
**Affects:** TPL-23, CNS-26, `tests/templates/declarations/member-outside-clause-missing.cpp`
**Source:** finding W17-4 of `reports/clean-room/W17/report.md`, from the probes that a clean-room writer ran for it.

## Context

A class template has the head `template <Small T> requires (sizeof(T) > 1)`.
A member function is defined after the class body. The definition's
head leaves out part of the class's head. TPL-23 says that the
definition starts with the class's template head, and CNS-26 asks for
equivalent constraints in the same places.

| The definition's head leaves out | Clang 22, GCC 16 | EDG 7.0 |
|---|---|---|
| the requires clause, and keeps the type constraint | reject | accept |
| the requires clause of a head that has no type constraint | reject | accept |
| the type constraint | reject | reject |
| nothing | accept | accept |

EDG 7.0 also accepts the first case when the program calls the member.
Clang 18 and GCC 14 reject as Clang 22 and GCC 16 do.

## Evidence

The test in **Affects**, with an override for EDG.
`tests/templates/declarations/member-outside-constrained.cpp` repeats
the whole head, and every compiler accepts it.

## Options

1. The definition repeats the whole head, with the requires clause, and
   a definition without the clause is rejected (Clang, GCC).
2. A definition can leave out the requires clause of the class's head
   (EDG).

## Provisional answer

The definition repeats the whole head, requires clause included
(option 1). Two of three families agree. CNS-26 treats a requires
clause the same way as a type constraint, and EDG rejects a definition
that leaves out the type constraint.

Kept open on 2026-11-04 under decision 0054, because no review has read the question. Item RV2 of decision 0056 reads it.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
