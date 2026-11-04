# Q-0203: Can an explicit specialization of a variable template be thread_local when the template is not?

**Status:** provisional
**Affects:** TSP-24, TSP-25

## Context

`template <class T> int counter = 0;` and
`template <> thread_local int counter<char> = 1;`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts and runs |
| GCC 14, 16, host | accepts and runs |
| EDG 7.0 | rejects: a thread-local declaration follows one that is not |

All compilers accept the program when the template is `thread_local`
too, and all reject other storage class specifiers on an explicit
specialization.

## Evidence

`tests/templates/specialization/explicit-thread-local-only.cpp`, with
an override for EDG, and `explicit-thread-local.cpp`.

## Options

1. The explicit specialization takes its properties from its own
   declaration, so the difference is valid (Clang, GCC).
2. The explicit specialization must agree with the template on
   `thread_local` (EDG).

## Provisional answer

Yes; Clang and GCC accept, EDG rejects (option 1). Clang and GCC agree, and TSP-25 gives an explicit
specialization its `inline` and `constexpr` from its own declaration.

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 accept and run the program, and EDG 7.0 rejects it, so two families disagree. TSP-P1 supports the provisional answer. An explicit specialization is a separate definition and takes no `inline` or `constexpr` from the template (TSP-25), and `thread_local` follows the same model (TSP-24). EDG's behavior is on the cluster 5 list of EDG differences that waits on the maintainer (`documentation/decisions/pending.md`).

Kept on 2026-11-03. The source-reader gap review of question cluster 5 agrees with option 1, with doubt.

Kept open on 2026-11-03 under decision 0054, because the gap review does not support the answer with no doubt (it reads with doubt, weakly, unclearly, or for only part of the question).

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 5 (`reports/review/2026-11-03-gap-review-question-cluster-5.md`) reads the reference as supporting option 1, with doubt. An explicit specialization can carry thread-local storage and no other storage class. The reference does not say whether the declaration of the variable template counts as a declaration of the explicit specialization for the rule that every declaration repeats thread-local storage. The reference treats a specialization as an entity of its own elsewhere, which supports option 1. The doubt is that no example covers the mixed case.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review of cluster 6, 2026-11-03

The source-reader gap review of question cluster 6 (`reports/review/2026-11-03-gap-review-question-cluster-6.md`) names the question only in its row for Q-0233. The review says that an explicit instantiation may have to repeat `thread_local` if the template's declaration counts as a declaration of the specialization, and calls that the doubt already raised for this question. It gives no new reading.
