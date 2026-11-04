# Q-0027: May a member function with a ref-qualifier be overloaded with one without?

**Status:** provisional
**Affects:** CLS-6, `tests/classes/members/ref-qualified-mixed.cpp`

## Context

`struct S { void f() &; void f(); };` declares one overload of `f` with a
ref-qualifier and one without.

| Compiler | Result |
|---|---|
| Clang 22, 18 | rejects the declarations |
| GCC 14 | rejects: "cannot be overloaded with 'void S::f() &'" |
| GCC 16 | accepts the declarations; a call `s.f()` is ambiguous |
| EDG 7.0 | rejects the declarations |

## Options

1. The declarations are rejected.
2. The declarations are valid, and a call that can't choose between them
   is rejected, as GCC 16 does.

## Provisional answer

No; the declarations are rejected (option 1). Three implementations, including the older GCC, reject the
declarations. GCC 16's change is recorded here so contributors can say
whether it reflects a deliberate language change.

Kept open on 2026-11-03 under decision 0054, because no principle on the page states the case. CLS-P3 says that a ref-qualifier selects a function by the value category of the object, and says nothing about a mix of overloads with and without one. GCC 16 is the one newest family that differs.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) agrees with option 1. Two member functions with the same parameters, one with a ref-qualifier and one without, have matching object parameters once references are set aside, so the two declarations clash. GCC 16's acceptance is a defect, and the review knows of no later change that allows the pair.
