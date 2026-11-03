# Q-0163: Does the result tie-breaker prefer the identity to a function pointer conversion?

**Status:** settled
**Affects:** OVL-26, `tests/foundations/overload-resolution/tie-conversion-result-templates.cpp`, `tests/foundations/overload-resolution/tie-conversion-result-noexcept.cpp`

## Context

A class has two conversion functions. One gives `int (*)()`, the other
`int (*)() noexcept`. The class initializes an `int (*)()`. The first
result reaches the target with the identity, and the second with a
function pointer conversion.

| Compiler | Two plain functions | Two templates | Template `int (*)()`, plain `noexcept` |
|---|---|---|---|
| Clang 18, Clang 22 | the identity | the identity | the template, by the result |
| GCC 14, GCC 16 | the identity | the identity | the template, by the result |
| EDG 7.0 | ambiguous | ambiguous | the plain function |

For arguments, every compiler prefers the identity to a function pointer
conversion.

## Evidence

`tests/foundations/overload-resolution/tie-conversion-result-noexcept.cpp`
and `tie-conversion-result-templates.cpp`. Item O-3 of the gap review of
overload resolution
(`reports/review/2026-10-31-gap-review-overload-resolution.md`)
describes the identity as the better result.

## Options

1. The result sequences are compared as for arguments, so the identity
   wins (Clang, GCC).
2. A function pointer conversion ties with the identity in the result
   (EDG).

## Answer

Yes; Clang and GCC agree, EDG differs (option 1). Two of three implementations agree, the gap review describes
it, and the answer matches the ranking of arguments.

The question stays open under decision 0051 (2026-11-03). EDG 7.0 finds the two results ambiguous, and Clang 22 and GCC 16 prefer the identity, so two families disagree. The provisional answer follows ICS-26. The identity is a subsequence of a function pointer conversion, so the identity is better, as every compiler finds for arguments (`subsequence-function-pointer.cpp`). OVL-26 compares the results as standard sequences. Item O-3 of the gap review gives the same answer.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of question cluster 2 (`reports/review/2026-11-03-gap-review-question-cluster-2.md`) supports this answer with no doubt, and OVL-P3 gives the same answer: step 2a compares the two results as standard sequences (ICS-P5), and the identity is better than a function pointer conversion. EDG 7.0 still differs. Tidal records EDG's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for EDG.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 2 (`reports/review/2026-11-03-gap-review-question-cluster-2.md`) agrees with option 1. The result tie-breaker compares two standard sequences by the usual steps. The identity is a subsequence of every sequence that is not the identity, and a function pointer conversion is not the identity. The result tie-breaker also comes before the preference for a plain function over a template, which fits the third column.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
