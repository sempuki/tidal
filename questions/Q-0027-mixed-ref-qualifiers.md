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

When the two overloads differ in cv-qualifiers, as `int f() const &;`
and `int f();`, Clang 22, GCC 16, and EDG 7.0 accept the pair, and GCC
14 rejects it (`tests/classes/members/ref-qualified-mixed-different-cv.cpp`).

## Options

1. The declarations are rejected.
2. The declarations are valid, and a call that can't choose between them
   is rejected, as GCC 16 does.

## Provisional answer

No; the declarations are rejected (option 1). Three implementations, including the older GCC, reject the
declarations. GCC 16's change is recorded here so contributors can say
whether it reflects a deliberate language change.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page states the case. CLS-P3 says that a ref-qualifier selects a function by the value category of the object, and says nothing about a mix of overloads with and without one. GCC 16 is the one newest family that differs.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). CLS-P3 failed step 3 of the test, because the model said nothing of a mix of overloads with and without a ref-qualifier. Item PQ widened CLS-P3. A function with no ref-qualifier takes an object of either category and would compete with each qualified overload, so overloads with the same parameters all have ref-qualifiers or none. CLS-26 and its two tests check the case. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses CLS-P3.

Used on 2026-11-04 under decision 0056 by the source-reader gap review of the text phase 8 wrote on classes and overloading (`reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`). The review applies CLS-P3 to the program of `ref-qualified-mixed.cpp` without the rule text and reaches the provisional answer: the two overloads have the same parameters and one has a ref-qualifier, so the declarations are rejected.

Kept open on 2026-11-04 under decision 0055, because CLS-P3 changed in the same commit: the review found that the model left out the cv-qualifiers of the object, and CLS-P3 and CLS-26 now limit the case to overloads with the same cv-qualifiers. The question waits for the next use of CLS-P3.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) agrees with option 1. Two member functions with the same parameters, one with a ref-qualifier and one without, have matching object parameters once references are set aside, so the two declarations clash. GCC 16's acceptance is a defect, and the review knows of no later change that allows the pair.

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on classes and overloading (`reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`) agrees with option 1. Two member functions with one name correspond when their other parameters match and their object parameters correspond. With one function lacking a ref-qualifier, the object types must still match once references are set aside. The review adds that CLS-P3 does not say the cv-qualifiers count: a `const` function with a ref-qualifier can be overloaded with one that has neither. Q-0027's program has no cv-qualifiers, so the answer stands.
