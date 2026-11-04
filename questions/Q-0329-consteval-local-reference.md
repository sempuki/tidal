# Q-0329: Does the initializer of a local `const` reference that binds a temporary run during constant evaluation?

**Status:** provisional
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
(`consteval-local-initializers.cpp`).

## Options

1. The initializer runs at run time, because the reference binds a
   local temporary and is not a constant initializer. `r` is 2.
2. The implementation tries the initializer as a constant, as for a
   `const` integer variable. `r` is 1.

## Provisional answer

The initializer runs at run time and `r` is 2 (option 1).
Option 1 follows CE-P2: a constant evaluation cannot give a reference
to a local temporary. Clang behaves this way, and GCC differs.

Reopened as provisional on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) supports this answer with no doubt. CE-P2 fails the test. CE-P2 is a principle of the constant evaluation page, and no principle or rule of the `if` page cites CE-P2. The `if` page cites CE-P2 only in its Evidence. The model of CE-P2 also lets a variable with a constant initializer keep a `const` temporary alive, so option 2 also fits the model. No other principle on the page gives the answer. GCC 16 differs, and the tests keep their overrides for GCC 16. EDG does not run the program.

On 2026-11-04, item PQ widened IF-P4 to state the case and to cite CE-P2. IF-P4 now lists the places where an initializer runs in constant evaluation, and says that the initializer of a local reference that binds a temporary runs at run time, because the result of a constant evaluation cannot refer to an automatic temporary. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses IF-P4.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) agrees with option 1. A local reference bound to a temporary refers to an object of automatic storage, which is not a valid result of a constant expression. So the reference is neither usable in constant expressions nor constant-initialized, and its initializer is not a constant evaluation. GCC's result is a defect.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
