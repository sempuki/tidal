# Q-0086: Is a call to a function that returns an abstract class valid?

**Status:** settled
**Affects:** VIRT-8, `tests/classes/virtual/call-returning-abstract.cpp`

## Context

A function is declared to return an abstract class, and a program calls it. GCC and EDG reject the call. Clang accepts it.

## Options

1. The call is not valid, because the call creates an object of the abstract class.
2. The call is valid.

## Answer

No; the call creates an object of the abstract class (option 1). GCC and EDG agree.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page states the case. VIRT-P4 says that an abstract class cannot be an object on its own, and does not say whether the result of a call to a function that returns such a class is an object. Clang 22 is the one newest family that differs.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). VIRT-P4 failed step 3 of the test, because the model did not say whether a call makes an object. Item PQ widened VIRT-P4 with a table of the constructs that make an object of an abstract class and the constructs that only name the class. A call makes its result as an object of the return type, so the call is rejected while the declaration of the function stays valid. The tests of VIRT-8 check each row. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses VIRT-P4.

Used on 2026-11-04 under decision 0056 by the source-reader gap review of the text phase 8 wrote on classes and overloading (`reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`). The review applies VIRT-P4 to the program of `call-returning-abstract.cpp` without the rule text and reaches the provisional answer: a call makes its result as an object of the abstract class, so the call is rejected, and the declaration stays valid.

Kept open on 2026-11-04 under decision 0055, because VIRT-P4 changed in the same commit: the review found that its table left out an argument for a parameter of abstract class type in a call. The question waits for the next use of VIRT-P4.

Used on 2026-11-04 by the principle-use batch PU1 (`reports/clean-room/PU1/`), under decision 0056. A clean-room reader applied VIRT-P4, without the rule text, to the program of `call-returning-abstract.cpp`. The answer names VIRT-P4 and gives the outcome that the test expects, so the use supports the answer.

Settled on 2026-11-04 under decision 0054. The use in batch PU1 supports the answer, the gap review of 2026-11-04 supports it with no doubt, and VIRT-P4 gives the same answer by the test of the style guide (section 3). Clang 22 still differs, and the test keeps its override.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) agrees with option 1. A function can be declared with an abstract return type. A call gives a prvalue of that type, and a prvalue of abstract class type is allowed only in the operand of `decltype`. Clang's acceptance is a defect.

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on classes and overloading (`reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`) agrees with option 1. A prvalue of abstract class type is not allowed outside one kind of unevaluated operand, and a call that returns the class makes one. The review adds that the reference also names an abstract parameter type in a function being called, a row that VIRT-P4 and VIRT-8 did not have.
