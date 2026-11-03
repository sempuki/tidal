# Q-0231: Is the initializer of an inline static data member instantiated with its class?

**Status:** settled
**Affects:** INST-5, `tests/templates/instantiation/class-inline-static-member-unused.cpp`

## Context

`static inline int count = T::nothing;` in a class template, with
`Box<int>` defined as an object and `count` not used.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts |
| GCC 14, 16, host | accepts |
| EDG 7.0 | rejects: the initializer is not valid for `int` |

## Evidence

The test above. Inventory item H18 says that a static data member is
instantiated, with its initialization, only when its definition is
needed.

## Options

1. The initializer waits for a use (Clang, GCC).
2. The initializer is made with the class (EDG).

## Answer

No; Clang and GCC agree, EDG differs (option 1). Two of three implementations and the inventory agree.

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 accept the class with the unused member, and EDG 7.0 rejects the initializer, so two families disagree. INST-P2 supports the provisional answer. Its table makes only the declaration of a `static` data member with the class, and INST-P1 waits for a use that needs the definition (INST-5, INST-8). EDG's behavior is on the cluster 5 list of EDG differences that waits on the maintainer (`documentation/decisions/pending.md`).

Kept on 2026-11-03. The source-reader gap review of question cluster 5 agrees with option 1.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of question cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`) supports this answer with no doubt, and INST-P2 gives the same answer: the instantiation of a class makes only the declaration of a `static` data member. EDG 7.0 still differs. Tidal records EDG's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for EDG.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`) reads the reference as supporting option 1. The instantiation of a class instantiates the declarations of its static data members but not their definitions, and the initializer of an inline static member is part of the definition.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
