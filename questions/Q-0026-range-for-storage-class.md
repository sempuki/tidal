# Q-0026: May a range-based for variable have a storage class?

**Status:** settled
**Affects:** RFOR-5, `tests/statements/range-for/storage-class.cpp`, `tests/statements/range-for/static-thread-local-declaration.cpp`

## Context

`for (static int v : a)` and `for (thread_local int v : a)` are rejected by
Clang 22 and 18 ("loop variable may not be declared 'static'"), GCC 16
("for-range-declaration cannot be 'static'"), and EDG 7.0. GCC 14.2 accepts
both.

## Options

1. Rejected. GCC 14's acceptance is a defect fixed in a later GCC.
2. Accepted.

## Answer

No; GCC 14's acceptance is fixed in GCC 16 (option 1). The newest version of every implementation in the matrix
rejects both forms.

Settled on 2026-11-01 under decision 0051, because the newest version of each baseline family (Clang 22, GCC 16, EDG 7.0) agrees with this answer. GCC 14 still accepts both forms, and the tests keep their overrides for GCC 14.

## Opinions

### Gap review, 2026-11-01

The source-reader gap review of question cluster 1 (`reports/review/2026-11-01-gap-review-question-cluster-1.md`) supports the answer. The declaration of a range-based `for` loop allows only type specifiers and `constexpr`, so `static` and `thread_local` are not allowed.
