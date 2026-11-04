# Q-0271: Can a pack expansion give an argument to a parameter of an alias template that is not a pack?

**Status:** settled
**Affects:** PACK-11, `tests/templates/packs/expansion-to-alias-parameters.cpp`

## Context

`template <class A, class B> using Second = B;` and then
`Second<T...>` in a template with a type pack `T`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects: a pack expansion is used as an argument for a parameter of an alias template that is not a pack |
| GCC 14, 16, host | rejects: a pack expansion argument for a parameter that is not a pack |
| EDG 7.0 | rejects: a pack expansion cannot be used as that argument |

A class template accepts the same expansion (PACK-10).

## Evidence

The test above. Item 7 of the gap review `reports/review/2026-10-31-gap-review-packs-and-ctad.md` says that its reference
has no explicit statement that forbids the expansion, and that the
treatment there looks unsettled.

## Options

1. The expansion is not valid for an alias template, as every compiler
   rejects it.
2. The expansion is valid, as for a class template, and the arguments
   are matched after substitution.

## Answer

No; every compiler rejects (option 1). Every implementation rejects the expansion. An alias template
is replaced by its type at the use, so the type needs each argument
before the length of the pack is known.

Settled on 2026-11-01 under decision 0051, because the newest version of each baseline family (Clang 22, GCC 16, EDG 7.0) agrees with this answer. No older version differs. The doubt of the gap review (item 7) stays a ground for a contributor to reopen the question.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-11-01

The source-reader gap review of question cluster 1 (`reports/review/2026-11-01-gap-review-question-cluster-1.md`) finds the reference silent. No rule forbids a pack expansion in the arguments of an alias template when the matching parameter is not a pack, and no rule allows it. The text on aliases does not settle when substitution happens. The settled answer rests on the agreement of every compiler.
