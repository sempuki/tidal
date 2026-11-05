# Q-0174: Is a partial specialization valid when it is not more specialized than the primary template?

**Status:** provisional
**Affects:** TSP-11
**Waits for:** newest families that agree

## Context

`template <class T, class U> struct Pair;` and the partial
specialization `template <class T, class U> struct Pair<U, T>`. The
argument list only swaps the parameters, so the partial specialization
matches every argument list.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects: not more specialized than the primary template |
| GCC 14, 16, host | accepts |
| EDG 7.0 | accepts |

All compilers reject a partial specialization whose argument list is
the primary template's parameter list in the same order.

## Evidence

`tests/templates/specialization/partial-reordered.cpp`, with overrides
for GCC and EDG.

## Options

1. The partial specialization must be more specialized than the primary
   template, and the implementation diagnoses it (Clang).
2. The declaration is valid (GCC, EDG).

## Provisional answer

No; Clang rejects, GCC and EDG accept (option 1). A partial specialization that matches every argument list
takes the place of the primary template, which TSP-11 already forbids
for the plain parameter list. The check needs partial ordering, which
roadmap item O specifies.

Kept open on 2026-11-03 under decision 0054, because two of the newest families differ from the answer, and the rule records one differing family. GCC 16 and EDG 7.0 accept the declaration.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) agrees with option 1. Every partial specialization must be more specialized than its primary template. With the two parameters swapped, deduction succeeds both ways, so neither is more specialized. The rule is not exempt from diagnosis, so GCC's and EDG's acceptance is a defect.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
