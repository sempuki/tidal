# Q-0059: Can a conversion function have a deduced return type?

**Status:** provisional
**Affects:** CVF-1, `tests/classes/conversion-functions/deduced-auto.cpp`

## Context

A conversion function can be written as `operator auto()`, with the
target type deduced from its `return` statements. Clang 18 and 22, EDG
7.0, and GCC 14 accept it. GCC 16 rejects it under `-pedantic-errors`
and accepts it otherwise.

## Options

1. A conversion function can have a deduced return type.
2. The target type must be written.

## Provisional answer

Yes; a conversion function can have a deduced return type (option 1). Most implementations accept `operator auto()`, and GCC 16
rejects it only as a pedantic diagnostic.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page states the case. CVF-P1 says that a conversion function declares no return type because its name gives the type, and does not say whether the name can be a placeholder. GCC 16 differs only under `-pedantic-errors`.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). CVF-P1 failed step 3 of the test, because the model did not say whether the name of a conversion function can be a placeholder. Item PQ widened CVF-P1. The name can be `auto`, and the target type comes from the `return` statements. CVF-1 and `deduced-auto.cpp` check the case. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses CVF-P1.

Used on 2026-11-04 under decision 0056 by the source-reader gap review of the text phase 8 wrote on classes and overloading (`reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`). The review applies CVF-P1 to the program of `deduced-auto.cpp` without the rule text and reaches the provisional answer: the name of the conversion function can be `auto`, and the `return` statement gives the target type, so the program runs.

Kept open on 2026-11-04 under decision 0055, because CVF-P1 changed in the same commit: the review found that the model left out the limit on conversion function templates (Q-0388). The question waits for the next use of CVF-P1.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) agrees with option 1. Only a conversion function template is barred from a deduced return type, so a plain conversion function can deduce its type. GCC 16's pedantic error is a defect.

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on classes and overloading (`reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`) agrees with option 1. A plain conversion function can have a deduced return type. The review adds that a conversion function template cannot, and that neither CVF-P1 nor CVF-1 stated that limit. The answer to this question is not affected.
