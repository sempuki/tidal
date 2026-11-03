# Q-0286: Can a structured binding declaration have a constrained placeholder?

**Status:** settled
**Affects:** CPT-37, `tests/templates/concepts/placeholder-structured-binding.cpp`

## Context

A function declares `Any auto [a, b] = Pair{1, 2};`, where the concept
`Any` is always satisfied.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects: cannot be declared with constrained 'auto' |
| GCC 14, 16, host | rejects: structured binding declaration cannot have constrained 'auto' type |
| EDG 7.0 | accepts |

## Evidence

The test above. GCC gives its error as a pedantic diagnostic. Every
compiler rejects a constrained placeholder in a functional cast
(`placeholder-functional-cast.cpp`).

## Options

1. Not valid (Clang, GCC). A structured binding declaration takes only
   the plain placeholder.
2. Valid, with the constraint checked on the deduced type (EDG).

## Answer

No; Clang and GCC agree, EDG differs (option 1). Two of three implementations agree, and the gap review of
concepts and constraints (item 17) describes the same rule.

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 reject the constrained placeholder, and EDG 7.0 accepts it, so two families disagree. CPT-P1 supports the provisional answer. Its table lists the places where a constraint can be written, and CPT-37 is the exception that keeps a constrained placeholder out of a structured binding declaration. EDG's behavior is on the cluster 5 list of EDG differences that waits on the maintainer (`documentation/decisions/pending.md`).

Kept on 2026-11-03. The source-reader gap review of question cluster 5 agrees with option 1.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of question cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`) supports this answer with no doubt, and CPT-P1 gives the same answer: its table lists the places where a constraint can be written, and CPT-37 keeps a constrained placeholder out of a structured binding declaration. EDG 7.0 still differs. Tidal records EDG's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for EDG.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`) reads the reference as supporting option 1. A structured binding declaration allows only a short list of specifiers, and an example of the reference rejects the constrained placeholder.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
