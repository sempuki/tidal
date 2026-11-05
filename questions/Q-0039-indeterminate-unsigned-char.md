# Q-0039: Is copying an uninitialized `unsigned char` undefined?

**Status:** provisional
**Affects:** INIT-1, `tests/declarations/initialization/forms/copy-indeterminate-unsigned-char.cpp`, `copy-indeterminate-unsigned-char-run.cpp`

## Context

A gap review summary says that copying an uninitialized `unsigned char` (or `std::byte`) into another object of the same kind is not undefined; the copy just holds an unknown value. During constant evaluation, every compiler in the matrix rejects the copy. At run time no compiler shows a difference, so the test can't tell the two answers apart.

## Options

1. Not undefined at run time for `unsigned char` and `std::byte`; rejected during constant evaluation, as every compiler does.
2. Undefined for every type, as INIT-1 says now.

## Provisional answer

Not undefined at run time for `unsigned char` and `std::byte`, and rejected during constant evaluation (option 1). The constant-evaluation behavior is observed, and the run-time exception comes from the review summary. A test that can observe the run-time difference would settle the question.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page gives the answer. INIT-P2 says that an automatic object with no initializer has no value, and it does not state the exception for `unsigned char` and `std::byte`.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). INIT-P2 fails step 3, because it says only that an automatic object with no initializer has no value, so option 2 also fits. Item PQ widened INIT-P2: a read of such a value is undefined, a copy into an `unsigned char` or `std::byte` object is not, the copy also has no value, and constant evaluation rejects even that copy. The run-time half of the answer still rests on the review, because no test can observe it. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses INIT-P2.

Used on 2026-11-04 by the source-reader gap review of the text phase 8 wrote on the declarations and statements chapters (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`), under decision 0056. The review applied INIT-P2 to the program of `copy-indeterminate-unsigned-char.cpp` and `copy-indeterminate-unsigned-char-run.cpp` without the rule text and reached the provisional answer: the copy into a byte object is defined at run time, and constant evaluation rejects it.

Kept open on 2026-11-04 under decisions 0054 and 0055, because INIT-P2 and INIT-1 changed in this commit: the copied value itself must have the type `unsigned char` or `std::byte`. The question waits for the next use of the changed text.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. Producing an indeterminate value is undefined, except for the unsigned ordinary character type and `std::byte` when the value is copied into an object of such a type. The copy then holds an indeterminate value. Constant evaluation forbids a read of an object whose value is indeterminate, so the constant form needs a diagnostic. The review notes that a later change makes many such reads of uninitialized automatic variables erroneous behavior. That change matters for INIT-1 as a whole and not for this exception.

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on the declarations and statements chapters (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`) agrees with option 1. The program copies an `unsigned char` with no value into an `unsigned char`, which the reference leaves defined, and constant evaluation rejects the read. The review found that INIT-P2 named only the type of the target. The reference also needs the copied value itself to have the type `unsigned char` or `std::byte`. A later standard changes the treatment of such reads, and that change does not apply to C++23.
