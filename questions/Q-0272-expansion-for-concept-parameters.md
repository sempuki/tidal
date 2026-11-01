# Q-0272: Can a pack expansion give an argument to a parameter of a concept that is not a pack?

**Status:** provisional
**Affects:** PACK-11, `tests/templates/packs/expansion-to-concept-parameters.cpp`

## Context

`template <class A, class B> concept Two = true;` and then `Two<T...>`
in a template with a type pack `T`, also in a requires clause.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects at the definition: a pack expansion is used as an argument for a parameter of a concept that is not a pack |
| GCC 14, 16, host | rejects at the definition: a pack expansion argument for a parameter that is not a pack |
| EDG 7.0 | accepts, and `Two<int, char>` through the expansion is true |

## Evidence

The test above. Item 8 of the gap review `reports/review/2026-10-31-gap-review-packs-and-ctad.md` predicts a rejection, as
for an alias template (Q-0271).

## Options

1. The expansion is not valid for a concept, as for an alias template
   (Clang, GCC).
2. The expansion is valid, and the arguments are matched after
   substitution (EDG).

## Provisional answer

Option 1. Two of three implementations agree, and the gap review
predicts the same result.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
