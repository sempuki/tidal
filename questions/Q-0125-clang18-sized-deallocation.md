# Q-0125: Does Clang 18 omitting the sized deallocation functions change NEW-19?

**Status:** settled
**Affects:** NEW-19, `tests/expressions/new-delete/global-functions-by-name.cpp`, `tests/expressions/new-delete/aligned-forms-by-name.cpp`

## Context

NEW-19 says that every program has the global allocation and
deallocation functions, including the sized forms of `operator delete`
and `operator delete[]`, and can call them by name. Clang 22, GCC 14
and 16, and EDG accept a call of the sized forms with no declaration in
the program. Clang 18 rejects it, because it does not declare the sized
forms unless a flag asks for them.

## Evidence

`tests/RESULTS.md`, row `global-functions-by-name.cpp`.

## Options

1. Keep NEW-19 as written. Clang 18 differs, and Clang 22 agrees with
   the other compilers.
2. Leave the sized forms out of NEW-19.

## Answer

No; Clang 22 declares them (option 1). The newest version of every implementation in the matrix
declares the sized forms.

Settled on 2026-11-01 under decision 0051, because the newest version of each baseline family (Clang 22, GCC 16, EDG 7.0) agrees with this answer. Clang 18 still rejects the calls of the sized forms, and the tests keep their overrides for Clang 18.

## Opinions

### Gap review, 2026-11-01

The source-reader gap review of question cluster 1 (`reports/review/2026-11-01-gap-review-question-cluster-1.md`) supports the answer. The sized deallocation functions, for arrays and single objects, are among the functions that every translation unit declares implicitly.
