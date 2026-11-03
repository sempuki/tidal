# Q-0268: Are the constrained friend definitions of two class specializations two functions?

**Status:** settled
**Affects:** CNS-32, `tests/templates/constraints/friend-definition-per-specialization.cpp`, `tests/templates/constraints/friend-distinct-from-namespace.cpp`

## Context

`Box<int>` and `Box<double>` each define `friend int touch(int)
requires (sizeof(T) > 1)`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts |
| GCC 14, 16, host | accepts |
| EDG 7.0 | rejects: touch has already been defined |

## Evidence

The first test above. In `friend-distinct-from-namespace.cpp`, a namespace template declares `touch` with the same parameters and a constraint that does not use `T`. Clang and GCC find a call ambiguous between the friend and that template. EDG accepts the call, so it treats the two as one template. INST-13 makes two definitions of an unconstrained friend not valid.

## Options

1. Two functions (Clang, GCC). The clause names `T`, so the two
   declarations have different constraints.
2. One function, defined twice (EDG).

## Answer

Yes; Clang and GCC agree, EDG differs (option 1). Two of three implementations agree, and CNS-31 requires the friend to be a definition for this reason.

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 accept the two definitions, and EDG 7.0 rejects the second, so two families disagree. CNS-P5 supports the provisional answer. Each specialization of the class defines its own constrained friend, which no declaration outside the class redeclares (CNS-32). EDG's behavior is on the cluster 5 list of EDG differences that waits on the maintainer (`documentation/decisions/pending.md`).

Kept on 2026-11-03. The source-reader gap review of question cluster 5 agrees with option 1.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of question cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`) supports this answer with no doubt, and CNS-P5 gives the same answer: each specialization of the class defines its own constrained friend. EDG 7.0 still differs. Tidal records EDG's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for EDG.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`) reads the reference as supporting option 1. Such a constrained friend has no link to a declaration in any other scope, so each specialization of the class has its own friend function.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
