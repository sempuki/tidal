# Q-0091: Is delete[] of a derived array through a base pointer rejected during constant evaluation?

**Status:** settled
**Affects:** NEW-5, `tests/expressions/new-delete/delete-derived-array-through-base-constant.cpp`

## Context

Constant evaluation creates an array of a derived class with
`new[]` and deletes it with `delete[]` through a pointer to the base,
which has a virtual destructor. Clang and EDG reject the evaluation. GCC
accepts it.

## Options

1. The behavior is undefined, so constant evaluation rejects the deletion.
2. Constant evaluation accepts the deletion.

## Answer

Yes; the behavior is undefined, so constant evaluation rejects the deletion (option 1). Clang and EDG agree, and VIRT-12 treats a
single object the same way (Q-0056).

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page gives the answer. NEW-P2 says that the array form of `delete` matches an array, and it does not say that the static type must match the dynamic type of the elements. GCC 16 is the only newest family that differs.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). NEW-P2 failed step 3, because it did not say that the pointer's type must match the elements. NEW-P2 now says that the array form steps through the elements by the size of the pointer's type, so `delete[]` through a pointer to a base is undefined, also with a virtual destructor. Constant evaluation rejects the undefined deletion. The existing tests check the case. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses NEW-P2.

Used on 2026-11-04 by the source-reader gap review of the text phase 8 wrote on expressions, foundations, and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`), Part B. The review applied NEW-P2 to the question's program, the array form of `delete` through a pointer to a base class, in constant evaluation (`delete-derived-array-through-base-constant.cpp`), without the rule text: the array form through a base pointer is undefined, so constant evaluation rejects it. That is the provisional answer, and the model rules out the other options.

Settled on 2026-11-04 under decision 0054. The review supports the answer with no doubt, and NEW-P2 gives the same answer under the test of the style guide (section 3). GCC 16 still differs, and the tests keep their overrides.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. For the array form of `delete`, the behavior is undefined when the dynamic type of the deleted object is not similar to the static type, and a virtual destructor does not change that. Undefined behavior stops constant evaluation, so a diagnostic is required. GCC's acceptance is a defect.

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on expressions, foundations, and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`) agrees with option 1. The reference makes an array `delete` undefined when the dynamic type of the array differs from the static type, and a virtual destructor makes no exception.
