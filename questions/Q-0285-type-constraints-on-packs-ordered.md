# Q-0285: Are two variadic templates ordered by the type constraints on their packs?

**Status:** settled
**Affects:** CPT-19, CNS-3, `tests/templates/concepts/type-constraint-pack-one-atom.cpp`

## Context

One function template has the head `<A... T>`, another `<AB... T>`,
where `AB<T>` is `A<T> && sizeof(T) < 100`. A call with two `int`
arguments satisfies both. Each type constraint on a pack is one fold
expression.

| Compiler | Result (`-std=c++23`) |
|---|---|
| Clang 18, 22, host | rejects: the call is ambiguous |
| GCC 14, 16, host | rejects: the call is ambiguous |
| EDG 7.0 (`--c++23`) | accepts: chooses the `AB` template |

## Evidence

The test above. A fold expression is one atomic constraint on every
compiler (`fold-atomic.cpp`). The gap review of concepts and
constraints (item 13) expects the call to be ambiguous in the C++20
mode, and notes that a later language mode can order the templates.

## Options

1. The fold expression is one atomic constraint, so neither template is
   more constrained (Clang, GCC).
2. The fold expressions are compared through their operands, so the
   `AB` template is more constrained (EDG).

## Answer

No; Clang and GCC agree, EDG differs (option 1). Two of three implementations agree in the C++23 mode, and
the rule matches CNS-3 for a fold expression that the program writes.

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 find the call ambiguous, and EDG 7.0 chooses the `AB` template in its C++23 mode, so two families disagree. CNS-3 supports the provisional answer. A fold expression is one atomic constraint, and every compiler treats a fold expression that the program writes that way (`fold-atomic.cpp`). The gap review of concepts and constraints says that a later language mode can order the templates, and Q-0007 keeps language versions after C++23 out of scope. Cluster 5 (EDG alone on templates) also lists the question.

Kept on 2026-11-03. The source-reader gap review of question cluster 3 agrees with option 1.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of question cluster 3 (`reports/review/2026-11-03-gap-review-question-cluster-3.md`) supports this answer with no doubt, and CNS-P1 gives the same answer: its step 3 makes every expression other than `&&`, `||`, and a concept-id one atomic constraint, so a fold expression is one atom. EDG 7.0 still differs. Tidal records EDG's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for EDG.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 3 (`reports/review/2026-11-03-gap-review-question-cluster-3.md`) reads the reference as supporting option 1. A type constraint on a pack gives a fold over the concept checks. In C++23, normalization splits only parentheses, conjunctions, disjunctions, and concept references, so a fold is one atomic constraint. The two folds are different expressions, and neither template subsumes the other. A later language version normalizes such folds through their operands, which gives EDG's result, and Q-0007 keeps that version out of scope.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
