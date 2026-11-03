# Q-0233: Can an explicit instantiation have a storage class or an attribute after the keyword template?

**Status:** provisional
**Affects:** INST-25, `tests/templates/instantiation/explicit-storage-class.cpp`, `tests/templates/instantiation/explicit-attribute-after.cpp`, `tests/templates/instantiation/explicit-thread-local.cpp`

## Context

`template static int get(int);` and `template [[nodiscard]] int
get(int);`.

`template thread_local int counter<int>;` repeats `thread_local` for
a variable template declared `thread_local`.

| Compiler | `static` | attribute after `template` | attribute before `template` | `thread_local` |
|---|---|---|---|---|
| Clang 18, 22, host | rejects | rejects | rejects | accepts |
| EDG 7.0 | rejects | rejects | rejects | rejects |
| GCC 14, 16, host | accepts | accepts | rejects | accepts |

## Evidence

The tests above and `explicit-attribute-before.cpp`. Item 34 of the gap
review `reports/review/2026-10-31-gap-review-dependent-names-and-instantiation.md`
gave the `thread_local` probe. Clang's diagnostic
list (clang-diag-sema) has "explicit instantiation cannot have a
storage class". Inventory item G1 forbids attributes and storage classes
other than `thread_local`.

## Options

1. `static` and the attribute are not valid, and `thread_local` is
   valid (Clang; EDG except for `thread_local`).
2. All are accepted (GCC).
3. No storage class specifier is valid (EDG).

## Provisional answer

No, except thread_local; Clang agrees, GCC and EDG each differ (option 1). Two of three implementations agree on each column, and the
inventory and the gap review (item 34) allow `thread_local`.

The question stays open under decision 0051 (2026-11-03). GCC 16 accepts `static` and the attribute after `template`, and EDG 7.0 rejects `thread_local`, so two families disagree. Probes of 2026-11-03 give the same split for `extern template static`, for `extern template thread_local`, and for an attribute after `struct` in `template struct [[deprecated]] Box<int>;`, which GCC 16 accepts and Clang 22 and EDG 7.0 reject. INST-P3 supports the provisional answer through INST-25. An explicit instantiation names a specialization that the template already declares, so the specifiers and attributes of that declaration apply, and only `thread_local` can repeat.

Kept on 2026-11-03. The source-reader gap review of question cluster 6 (`reports/review/2026-11-03-gap-review-question-cluster-6.md`) agrees with option 1. Its side note asks whether an explicit instantiation of a variable template declared `thread_local` must repeat the keyword. That depends on the doubt of Q-0203. INST-25 allows `thread_local` and does not require it.

Kept open on 2026-11-03 under decision 0054, because the gap review does not support the answer with no doubt (it reads with doubt, weakly, unclearly, or for only part of the question).

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 6 (`reports/review/2026-11-03-gap-review-question-cluster-6.md`) reads the reference as supporting option 1. No storage class except `thread_local` can appear on an explicit instantiation, and no attribute applies to one. An attribute before `template` has no place in the grammar, and an attribute after the class key is not allowed either. As a side note, a variable declared `thread_local` must have the keyword on every declaration, so the reference may require the explicit instantiation to repeat it. That depends on whether the template's declaration counts as a declaration of the specialization, the doubt of Q-0203.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
