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

No; Clang and GCC agree, EDG differs (option 1). Two of three implementations agree, and the gap review
predicts the same result.

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 reject the expansion at the definition, and EDG 7.0 accepts it, so two families disagree. PACK-P1 supports the provisional answer. Its exception PACK-11 treats a concept as it treats an alias template, and Q-0271 is settled for the alias template with every compiler rejecting. EDG's behavior is on the cluster 5 list of EDG differences that waits on the maintainer (`documentation/status.md`).

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
