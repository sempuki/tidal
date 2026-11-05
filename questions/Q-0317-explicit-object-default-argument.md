# Q-0317: Can an explicit object parameter have a default argument?

**Status:** settled
**Affects:** CLS-14, `tests/classes/members/explicit-object-default-argument.cpp`

## Context

A member function declares its explicit object parameter as
`this Gauge self = Gauge{}`.

| Compiler | Result |
|---|---|
| Clang 22, 18 | rejects |
| GCC 16, 14 | rejects |
| EDG 7.0 | accepts |

The object of a call always initializes the parameter (CLS-13), so a
default argument for it can never be used.

## Evidence

The test above. Clang says that the explicit object parameter cannot
have a default argument, and GCC says the same in other words.

## Options

1. The default argument is not valid.
2. The default argument is valid and has no effect.

## Answer

No (CLS-14); EDG accepts (option 1). Clang and GCC reject the declaration, and the default
argument could never take effect. The test expects a rejection and
records EDG's result with an override.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page states the case. CLS-P3 lists what a function with an explicit object parameter cannot be, and the list does not mention a default argument. EDG 7.0 is the one newest family that differs.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). CLS-P3 failed step 3 of the test, because its list of what the function cannot be left out the default argument, so option 2 also fitted the model. Item PQ widened CLS-P3. The object of each call initializes the explicit object parameter, so a default argument could never take effect, and the parameter cannot have one. CLS-14 and `explicit-object-default-argument.cpp` check the case. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses CLS-P3.

Used on 2026-11-04 under decision 0056 by the source-reader gap review of the text phase 8 wrote on classes and overloading (`reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`). The review applies CLS-P3 to the program of `explicit-object-default-argument.cpp` without the rule text and reaches the provisional answer: the object of each call initializes the explicit object parameter, so the parameter cannot have a default argument, and the declaration is rejected.

Kept open on 2026-11-04 under decision 0055, because CLS-P3 changed in the same commit for the cv-qualifier detail of Q-0027. The question waits for the next use of CLS-P3.

Used on 2026-11-04 by the principle-use batch PU1 (`reports/clean-room/PU1/`), under decision 0056. A clean-room reader applied CLS-P3, without the rule text, to the program of `explicit-object-default-argument.cpp`. The answer names CLS-P3 and gives the outcome that the test expects, so the use supports the answer.

Settled on 2026-11-04 under decision 0054. The use in batch PU1 supports the answer, the gap review of 2026-11-04 supports it with no doubt, and CLS-P3 gives the same answer by the test of the style guide (section 3). EDG 7.0 still differs, and the test keeps its override.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) agrees with option 1. The grammar of a parameter has no form with both the explicit object marker and a default argument. EDG's acceptance is a defect.

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on classes and overloading (`reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`) agrees with option 1. The grammar of a parameter has no form that carries both the marker of an explicit object parameter and a default argument.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
