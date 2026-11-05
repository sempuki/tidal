# Q-0329: Does the initializer of a local `const` reference that binds a temporary run during constant evaluation?

**Status:** settled
**Affects:** IF-25, IF-18, `tests/statements/if/consteval-local-reference.cpp`

## Context

IF-25 lists the places where an evaluation is a constant evaluation, and
so where the then branch of `if consteval` executes. The list includes
the constant initializer of a `const` local variable of integer or
enumeration type. CE-3 also makes a reference variable with a constant
initializer usable in constant expressions.

A local reference such as `const int& r = where();` binds a temporary
that the reference extends. The temporary is a local object. CE-P2 says
that the result of a constant evaluation cannot point to a local or
temporary object, so such an initializer is not a constant expression.

## Evidence

`consteval-local-reference.cpp` calls a `constexpr` function whose
`if consteval` gives 1 in its then branch and 2 in its else branch.
Clang 18 and Clang 22 give 2. GCC 14 and GCC 16 give 1. EDG translates
the program and does not run it. Both compiler families give 1 for a
local `const int` and 2 for a local `const double`
(`consteval-local-initializers.cpp`). Both also give 1 for a local
`const` enumeration variable, and 2 for a local `const volatile int` and
for a local variable that is not `const`.

## Options

1. The initializer runs at run time, because the reference binds a
   local temporary and is not a constant initializer. `r` is 2.
2. The implementation tries the initializer as a constant, as for a
   `const` integer variable. `r` is 1.

## Answer

The initializer runs at run time and `r` is 2 (option 1).
Option 1 follows CE-P2: a constant evaluation cannot give a reference
to a local temporary. Clang behaves this way, and GCC differs.

Reopened as provisional on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) supports this answer with no doubt. CE-P2 fails the test. CE-P2 is a principle of the constant evaluation page, and no principle or rule of the `if` page cites CE-P2. The `if` page cites CE-P2 only in its Evidence. The model of CE-P2 also lets a variable with a constant initializer keep a `const` temporary alive, so option 2 also fits the model. No other principle on the page gives the answer. GCC 16 differs, and the tests keep their overrides for GCC 16. EDG does not run the program.

On 2026-11-04, item PQ widened IF-P4 to state the case and to cite CE-P2. IF-P4 now lists the places where an initializer runs in constant evaluation, and says that the initializer of a local reference that binds a temporary runs at run time, because the result of a constant evaluation cannot refer to an automatic temporary. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses IF-P4.

Used on 2026-11-04 by the source-reader gap review of the text phase 8 wrote on the declarations and statements chapters (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`), under decision 0056. The review applied IF-P4 to the program of `consteval-local-reference.cpp` without the rule text and reached the provisional answer: the local reference binds a temporary, so its initializer runs at run time and the result is 2.

Kept open on 2026-11-04 under decisions 0054 and 0055, because IF-P4 and IF-25 changed in this commit for the two cases that the review names. The question waits for the next use of the changed text.

Used on 2026-11-04 by the principle-use batch PU1 (`reports/clean-room/PU1/`), under decision 0056. A clean-room reader applied IF-P4, without the rule text, to the program of `consteval-local-reference.cpp`. The answer names IF-P4 and gives the outcome that the test expects, so the use supports the answer.

Settled on 2026-11-04 under decision 0054. The use in batch PU1 supports the answer, the gap review of 2026-11-04 supports it with no doubt, and IF-P4 gives the same answer by the test of the style guide (section 3). GCC 16 still differs, and the test keeps its override. EDG does not run the program.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) agrees with option 1. A local reference bound to a temporary refers to an object of automatic storage, which is not a valid result of a constant expression. So the reference is neither usable in constant expressions nor constant-initialized, and its initializer is not a constant evaluation. GCC's result is a defect.

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on the declarations and statements chapters (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`) agrees with option 1. The temporary that the reference extends has automatic storage duration, so a constant expression cannot refer to it, and the initializer runs at run time. The review found that IF-P4 left out a local reference that binds an object with static storage duration, whose initializer is tried as a constant, and an initializer inside a function that a constant evaluation runs.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
