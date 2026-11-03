# Q-0280: Can two deduction guides have the same parameters and different results?

**Status:** provisional
**Affects:** CTAD-12, `tests/templates/class-template-deduction/guide-same-parameters.cpp`, `guide-template-same-parameters.cpp`

## Context

`Box(int) -> Box<long>;` and `Box(int) -> Box<short>;` are two guides
that are not templates. `template <class T> Box(T*) -> Box<long>;` and
`template <class T> Box(T*) -> Box<short>;` are two guide templates.

| Compiler | Guides that are not templates | Guide templates |
|---|---|---|
| Clang 18, 22, host | rejects: functions that differ only in the return type | accepts; a use is ambiguous |
| GCC 14, 16, host | rejects: an ambiguating new declaration | accepts; a use is ambiguous |
| EDG 7.0 | accepts | accepts; a use is ambiguous |

## Evidence

The tests above. Item 37 of the gap review `reports/review/2026-10-31-gap-review-packs-and-ctad.md` says that two guides
for one class template with equivalent parameter lists are not valid
if either is reachable from the other, even with different results. It
predicts a rejection of both pairs.

## Options

1. Two guides that are not templates with the same parameters are not
   valid. Two guide templates are valid, and a use can be ambiguous.
   Clang and GCC give this result.
2. Both pairs are not valid, as the gap review reads.
3. Both pairs are valid (EDG).

## Provisional answer

Templates yes, others no; Clang and GCC agree, EDG differs (option 1). The guides follow the rules for the overloading of functions
and function templates, and two of three implementations agree on each
pair. No compiler rejects the guide templates.

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 reject the two guides that are not templates, and EDG 7.0 accepts them, so two families disagree. Every compiler accepts the two guide templates and finds a use ambiguous. CTAD-P3 supports the provisional answer. The guides compete as constructors do, so two guides that are not templates with the same parameter types are two declarations of one function that differ only in their results (CTAD-12). EDG's behavior is on the cluster 5 list of EDG differences that waits on the maintainer (`documentation/decisions/pending.md`).

Kept on 2026-11-03. The source-reader gap review of question cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`) reads the reference as supporting option 2. For the guides that are not templates, the review agrees with Clang and GCC, so EDG's acceptance is a defect. For the guide templates, the review reads the pair as not valid, and every compiler accepts the pair and finds a use ambiguous. CTAD-P3 lets the guides compete as functions and function templates do, and two function templates that differ only in their results are two templates. No family follows the reading for the guide templates, so the specification follows the shared behavior until the maintainer decides, as for Q-0243 and Q-0275. The question is on the maintainer's list in `documentation/status.md`.

Kept open on 2026-11-03 under decision 0054, because the question waits on the maintainer (`documentation/decisions/pending.md`).

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`) reads the reference as supporting option 2. Two deduction guides for one class template with equivalent parameter lists are not valid when either is reachable from the other. The results play no part, and the rule has no exemption from diagnosis. The rule makes no exception for guide templates, so two guide templates with the same head and parameter list are not valid either. Clang and GCC reject only the pair that are not templates, through the rules for overloading, and EDG accepts both pairs.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
