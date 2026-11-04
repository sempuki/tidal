# Q-0110: Is a reference bound through a null pointer rejected during constant evaluation?

**Status:** settled
**Affects:** REF-5, `tests/declarations/initialization/references/null-dereference-bind-constant.cpp`

## Context

A constexpr function binds a reference to `*p` where `p` is a null pointer, and does not read through the reference. GCC and Clang 22 reject the evaluation. EDG 7.0 and Clang 18 accept it.

## Options

1. The binding is undefined, so constant evaluation rejects it.

2. Only a read through the reference is undefined.

## Answer

Yes; the binding is undefined, so constant evaluation rejects it (option 1). The newer Clang agrees with GCC.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on overloading, references, and list initialization (`reports/review/2026-11-03-gap-review-questions-overloading.md`) supports this answer with no doubt, and REF-P2 gives the same answer: a reference bound through a null pointer has no defined meaning, so constant evaluation rejects the binding. EDG 7.0 still differs. Tidal records EDG 7.0's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for EDG 7.0.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on overloading, references, and list initialization (`reports/review/2026-11-03-gap-review-questions-overloading.md`) agrees with option 1. Every reference must be bound to an existing object or function, and indirection through a null pointer is undefined. Constant evaluation rejects an undefined operation. A later change states the undefined indirection outright, and GCC and Clang 22 follow that reading.

