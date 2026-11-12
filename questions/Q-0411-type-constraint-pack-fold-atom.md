# Q-0411: Are the folds that one concept makes on a type parameter pack in two declarations one atom or two?

**Status:** provisional
**Affects:** CPT-19, CNS-4, `tests/templates/concepts/type-constraint-pack-same-concept-two-declarations.cpp`
**Source:** finding X22-3 of `reports/clean-room/X22/report.md`, from the probe `k1.cpp` beside it.
**Waits for:** a principle (CNS-P1)

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

Kept open on 2026-11-11 under decision 0054 (decision 0066, item DQ). The second review supports the answer with no doubt, and EDG 7.0 alone differs. No principle gives the answer. CNS-P1 says that an atom is known by its place in the source, and also that a concept-id becomes the normal form of its concept, so that declarations share its atoms. CNS-P1 does not say that the fold from a type constraint on a pack stays one atom, so option 2 also fits the model, and CNS-P1 fails step 3 of the principle test.

## Opinions

### Gap review, 2026-11-05 (phase 10 text, expressions and templates)

The source-reader gap review of the text phase 10 wrote on expressions and templates (`reports/review/2026-11-05-gap-review-phase-10-expressions-templates.md`) reads the question: supports with a doubt. In C++23 each type constraint on a pack gives its own fold, which is one atom. Two atoms are identical only when they come from one appearance, so neither declaration subsumes the other and the call is ambiguous. The doubt is that the fold has no written appearance, and a C++26 change may give EDG's answer.

### Second review of the doubt-held questions, 2026-11-11

The source-reader second review of the questions held only by a doubt (`reports/review/2026-11-11-second-review-doubt-held-questions.md`) reads the question: supports, and the doubt is resolved. In C++23 a fold that a type constraint on a pack makes is itself one atom. Each declaration writes its own type constraint, so the two folds come from two places in the source and are not identical. The later change that EDG may follow is not part of C++23.
