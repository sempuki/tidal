# Q-0257: Must an implementation reject a constant evaluation that needs a declaration after the first point of instantiation?

**Status:** provisional
**Affects:** INST-31, INST-33, `tests/templates/instantiation/function-point-constant.cpp`

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

## Provisional answer

Yes; every compiler rejects (option 1). Every compiler rejects, so a `reject` rule matches every implementation. Option 2 also allows every result, and a contributor can argue for it with the gap review.

The question stays open under decision 0051 (2026-11-01). Every compiler rejects, and a rejection also fits option 2, so the compilers do not separate the options. The gap review (item 31) reads the case against option 1.

## Opinions

### Gap review, 2026-11-01

The source-reader gap review of question cluster 1 (`reports/review/2026-11-01-gap-review-question-cluster-1.md`) reads the reference as supporting option 2: no diagnostic is required. A writer weighs the change with its rules and tests.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
