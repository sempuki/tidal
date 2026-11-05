# Q-0088: Does assignment through a user-provided operator activate an inactive union member during constant evaluation?

**Status:** settled
**Affects:** UNI-6, `tests/classes/unions/user-assignment-not-activating-constant.cpp`

## Context

A union member has a class type with a user-provided assignment operator. Constant evaluation assigns to that member while another member is active. Clang and GCC reject the evaluation. EDG 7.0 accepts it.

## Options

1. The member does not become active, so the access is undefined and constant evaluation rejects it.
2. The assignment makes the member active.

## Answer

No; the access is undefined, so constant evaluation rejects it (option 1). Clang and GCC agree.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page states the case. UNI-P1 says that a plain assignment to another member makes that member active, and does not say whether an assignment through a user-provided operator counts. EDG 7.0 is the one newest family that differs.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). UNI-P1 failed step 3, because "a plain assignment" did not say whether a user-provided operator counts. UNI-P1 now lists the two actions that make a member active, the built-in or trivial assignment among them, and says that a user-provided operator is a call on an object that is not there yet. The existing test checks the case. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses UNI-P1.

Used on 2026-11-04 under decision 0056 by the source-reader gap review of the text phase 8 wrote on classes and overloading (`reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`). The review applies UNI-P1 to the program of `user-assignment-not-activating-constant.cpp` without the rule text and reaches the provisional answer: the assignment calls a user-provided operator on a member that is not an object yet, so the member does not become active, the behavior is undefined, and constant evaluation rejects it.

Settled on 2026-11-04 under decision 0054. The source-reader gap review of the text phase 8 wrote on classes and overloading (`reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`) supports this answer with no doubt, and UNI-P1 gives the same answer by the test of the style guide (section 3). UNI-P1 did not change in the commit that recorded the use. EDG 7.0 still differs, and the test keeps its override for it.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) agrees with option 1. Only an assignment through the built-in operator or a trivial assignment operator changes the active member. A user-provided operator is a member call on an object whose lifetime has not begun, which is undefined.

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on classes and overloading (`reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`) agrees with option 1. Only the built-in assignment or a trivial assignment operator can start the lifetime of the member. A call of a member function on an object outside its lifetime is undefined.
