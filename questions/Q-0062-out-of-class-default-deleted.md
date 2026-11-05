# Q-0062: Must an implementation diagnose a comparison defaulted outside its class that would be deleted?

**Status:** provisional
**Affects:** DCMP-8, `tests/classes/comparisons/out-of-class-would-delete.cpp`

## Context

A class declares `operator==` and defaults it after the class
definition. A member of the class has no `==`, so the defaulted function
would be deleted. Clang rejects the definition. GCC and EDG accept it
while no code uses the function. For copy and move constructors, all
compilers in the matrix reject the same pattern (CPY-17).

## Options

1. The program is not valid, and an implementation need not diagnose it
   (`no-diagnostic`).
2. The program is not valid and must be rejected (`reject`).
3. The function is deleted, and only a use is rejected.

## Provisional answer

No; the program is not valid and must be rejected (option 2). Clang rejects the definition, and DCMP-8 matches CPY-17. GCC and EDG accept it.

Changed on 2026-11-03 to option 2. The review reads option 2, Clang 22 rejects, and DCMP-P1 now gives the same answer: a comparison defaulted after the class is a definition that the program asks for, so a definition that would be deleted is rejected, as CPY-17 rejects it for the copy and move functions on every compiler. DCMP-8 is now a `reject` rule. The tests already expected a rejection, with overrides for GCC and EDG. The question stays open under decision 0054, because two newest families differ from the answer.

Kept open on 2026-11-04 under decision 0054, because two newest families, GCC 16 and EDG 7.0, differ from the answer.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) reads option 2. A function that the class declares and defaults later is defined where it is defaulted. If that definition would be deleted, the program is not valid, and nothing exempts the case from diagnosis, so a diagnostic is required also when nothing uses the comparison. Clang follows this reading. GCC and EDG accept.
