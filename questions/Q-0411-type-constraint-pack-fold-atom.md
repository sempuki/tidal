# Q-0411: Are the folds that one concept makes on a type parameter pack in two declarations one atom or two?

**Status:** provisional
**Affects:** CPT-19, CNS-4, `tests/templates/concepts/type-constraint-pack-same-concept-two-declarations.cpp`
**Source:** finding X22-3 of `reports/clean-room/X22/report.md`, from the probe `k1.cpp` beside it.
**Waits for:** a first review

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

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
