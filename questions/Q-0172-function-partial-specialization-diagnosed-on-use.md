# Q-0172: Must an implementation reject a partial specialization of a function template that no call uses?

**Status:** provisional
**Affects:** TSP-7

## Context

`template <class T> int pick<T, int>(T, int)` declares a partial
specialization of a function template.

| Program | Clang 18, 22, host | GCC 14, 16, host | EDG 7.0 |
|---|---|---|---|
| the declaration only | rejects | rejects | accepts |
| the declaration and a call of `pick` | rejects | rejects | rejects at the call |

## Evidence

`tests/templates/specialization/function-partial.cpp` (an override for
EDG) and `tests/templates/specialization/function-partial-used.cpp`.

## Options

1. The declaration is not valid, and the implementation diagnoses it
   (Clang, GCC).
2. The implementation need not diagnose it until a use (EDG).

## Provisional answer

Yes; Clang and GCC agree, EDG rejects only at a use (option 1). Clang and GCC reject the declaration itself, and EDG rejects
it as soon as the template is used. TSP-7 stays a `reject` rule.

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 reject the declaration with no call, and EDG 7.0 rejects only at a call, so two families disagree on the declaration alone. TSP-P1 supports the provisional answer. Its table gives a function template no partial specialization, so the declaration is not valid whether or not a call uses it (TSP-7). EDG's behavior is on the cluster 5 list of EDG differences that waits on the maintainer (`documentation/status.md`).

Kept on 2026-11-03. The source-reader gap review of question cluster 5 agrees with option 1.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`) reads the reference as supporting option 1. Only a class template or a variable template can have a partial specialization. The rule has no exemption from diagnosis, and it applies to the declaration itself.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
