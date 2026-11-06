# Q-0411: Are the folds that one concept makes on a type parameter pack in two declarations one atom or two?

**Status:** provisional
**Affects:** CPT-19, CNS-4, `tests/templates/concepts/type-constraint-pack-same-concept-two-declarations.cpp`
**Source:** finding X22-3 of `reports/clean-room/X22/report.md`, from the probe `k1.cpp` beside it.
**Waits for:** a review (doubt)

## Context

Two function templates each write `template <A... T>`, and the second
adds `requires B<int>`. CPT-19 makes each `A...` a fold `(A<T> && ...)`.
If the two folds are one atom, the second template's constraints
subsume the first's, and a call chooses the second. If they are two
atoms, neither subsumes the other, and the call is ambiguous.

| Implementation | Result |
|---|---|
| Clang 22 | ambiguous |
| GCC 16 | ambiguous |
| EDG 7.0 | chooses the second template |

## Evidence

The test in **Affects**, with an override for EDG.

## Options

1. Two atoms, one from each declaration where the type constraint is
   written, so the call is ambiguous (Clang, GCC).
2. One atom, so the second template is more constrained (EDG).

## Provisional answer

Two atoms (option 1). Two of three families agree, and CNS-4 makes two
atoms identical only when they come from the same place in the source.

Kept open on 2026-11-05 under decision 0054, because the review reads the answer with a doubt (gap review of the text phase 10 wrote, item RV4).

## Opinions

### Gap review, 2026-11-05 (phase 10 text, expressions and templates)

The source-reader gap review of the text phase 10 wrote on expressions and templates (`reports/review/2026-11-05-gap-review-phase-10-expressions-templates.md`) reads the question: supports with a doubt. In C++23 each type constraint on a pack gives its own fold, which is one atom. Two atoms are identical only when they come from one appearance, so neither declaration subsumes the other and the call is ambiguous. The doubt is that the fold has no written appearance, and a C++26 change may give EDG's answer.
