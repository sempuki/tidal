# Q-0257: Must an implementation reject a constant evaluation that needs a declaration after the first point of instantiation?

**Status:** settled
**Affects:** INST-31, INST-32, INST-33, `tests/templates/instantiation/function-point-constant.cpp`

## Context

A `constexpr` function template calls `probe(item)`, and `probe` is
declared after the constant that calls the function. The end of the
unit is also a point of instantiation, and there `probe` is declared.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects |
| GCC 14, 16, host | rejects |
| EDG 7.0 | rejects |

## Evidence

The test above. Item 31 of the gap review `reports/review/2026-10-31-gap-review-dependent-names-and-instantiation.md` says
that no rule limits a constant evaluation to the declarations before
the first point, and that no rule requires a rejection. The review places the case under the
no-diagnostic rule for points that differ (INST-32).

## Options

1. The program is not valid, and the implementation rejects it (every compiler).
2. The points differ, INST-32 applies, and no diagnostic is required.

## Answer

No; the points differ, INST-32 applies, and no diagnostic is required (option 2). Every compiler rejects, which INST-32 allows.

Changed on 2026-11-03. Item 31 of the earlier gap review and the source-reader gap review of question cluster 1 (`reports/review/2026-11-01-gap-review-question-cluster-1.md`) read the reference as supporting option 2. INST-P4 gives the same model on Tidal's own page. A function specialization has a point after its use and another at the end of the unit, and all the points must give it the same meaning. INST-31 now places the case under INST-32, and INST-33 covers only a class specialization. The test `function-point-constant.cpp` still expects a rejection, and its note names INST-32.

The question stays open under decision 0051 (2026-11-03). Every compiler rejects, and a rejection fits both options, so the compilers do not separate the options.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of question cluster 1 (`reports/review/2026-11-01-gap-review-question-cluster-1.md`) supports this answer with no doubt, and INST-P4 gives the same answer: a function specialization has a point after its use and another at the end of the unit, and the points must agree. No family differs. Every compiler rejects the program, which a `no-diagnostic` rule allows, so the test has no override.

## Opinions

### Gap review, 2026-11-01

The source-reader gap review of question cluster 1 (`reports/review/2026-11-01-gap-review-question-cluster-1.md`) reads the reference as supporting option 2: no diagnostic is required. A writer weighs the change with its rules and tests.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
