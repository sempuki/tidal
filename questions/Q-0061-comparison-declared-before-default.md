# Q-0061: Can a friend comparison declared outside the class be defaulted in it?

**Status:** settled
**Affects:** DCMP-3, `tests/classes/comparisons/declared-outside-then-defaulted.cpp`

## Context

A program declares a comparison operator at namespace scope. Then a class
declares the same function as a friend and defaults it. Clang and EDG
reject the default. GCC accepts it.

## Options

1. The first declaration of a defaulted comparison must be in its class.
2. An earlier declaration outside the class is valid.

## Answer

No; the first declaration of a defaulted comparison must be in its class (option 1). Clang and EDG agree.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page states the case. DCMP-P1 describes what a defaulted comparison does and does not say where its first declaration must be. GCC 16 is the one newest family that differs.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). DCMP-P1 failed step 3 of the test, because the model did not say where the first declaration of a defaulted comparison must be. Item PQ widened DCMP-P1. The class asks for a defaulted comparison in its own definition, so the comparison is a member or a friend that the class declares first, and defaulting a friend declared earlier outside the class is rejected. DCMP-3 and `declared-outside-then-defaulted.cpp` check the case. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses DCMP-P1.

Used on 2026-11-04 under decision 0056 by the source-reader gap review of the text phase 8 wrote on classes and overloading (`reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`). The review applies DCMP-P1 to the program of `declared-outside-then-defaulted.cpp` without the rule text and reaches the provisional answer: the friend was declared outside the class first, so the class did not ask for it, and defaulting it is rejected.

Settled on 2026-11-04 under decision 0054. The source-reader gap review of the text phase 8 wrote on classes and overloading (`reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`) supports this answer with no doubt, and DCMP-P1 gives the same answer by the test of the style guide (section 3). DCMP-P1 did not change in the commit that recorded the use. GCC 16 still differs, and the test keeps its override for it.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) agrees with option 1. A class can default a comparison only where the class declares that function for the first time. A friend that defaults a function already declared at namespace scope breaks that rule.

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on classes and overloading (`reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`) agrees with option 1. A defaulted comparison defined in a class must be the first declaration of that function, so a friend declared earlier at namespace scope cannot be defaulted in the class.
