# Q-0221: Is a `new` expression valid when its matching deallocation function is deleted?

**Status:** provisional
**Affects:** NEW-25, `tests/expressions/new-delete/deallocation-deleted.cpp`, `tests/expressions/new-delete/deallocation-deleted-no-throw.cpp`, `tests/expressions/new-delete/deallocation-placement-deleted.cpp`

## Context

A class declares its own `operator new` and a deleted
`operator delete` that matches it. A `new` expression creates an object
of the class.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects: the deleted function is used |
| EDG 7.0 | rejects: the deleted function cannot be referenced |
| GCC 14, 16, host | accepts |

The results are the same for a usual form and for a placement form.
The results are also the same when the initialization cannot throw.
All three compilers reject a private deallocation function in the same
cases.

## Evidence

The three tests above, each with an override for GCC.

## Options

1. The program is not valid (Clang, EDG). The deallocation function is
   checked as the access check already does.
2. The program is valid (GCC).

## Provisional answer

No; Clang and EDG reject, GCC accepts (option 1). Clang and EDG agree, and the access check, on which all
three agree, treats the deallocation function as used.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page gives the answer. NEW-P3 says which deallocation function pairs with an allocation after a throw, and no principle says that the `new` expression uses that function when nothing throws. GCC 16 is the only newest family that differs.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). NEW-P3 failed step 3, because it named the paired deallocation function only for a throw. NEW-P3 now says that the `new` expression uses that function also when the initialization cannot throw, so the function must be accessible and not deleted. The existing tests check the case. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses NEW-P3.

Used on 2026-11-04 by the source-reader gap review of the text phase 8 wrote on expressions, foundations, and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`), Part B. The review applied NEW-P3 to the question's program, a `new` expression whose deallocation function is deleted (`deallocation-deleted.cpp`, `deallocation-deleted-no-throw.cpp`, `deallocation-placement-deleted.cpp`), without the rule text: the expression uses the deallocation function in every case, so a deleted one makes it not valid, which is the provisional answer.

Kept open on 2026-11-04 under decision 0054, because the review finds the reference unclear. The reference checks access and ambiguity of the deallocation function for every allocation of class type, and it does not say outright that the selection is a use for the rule on deleted functions. The review leans to option 1. GCC 16 is the one newest family that differs.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. The single matching deallocation function for a class counts as named by the `new` expression, so the expression uses that function. A use of a deleted function makes the program not valid, whether or not the initialization can throw. GCC's acceptance is a defect.

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on expressions, foundations, and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`) finds the reference unclear and leaning to option 1. The reference checks access and ambiguity on the deallocation function of an allocation of class type whether or not the initialization can throw, and every compiler agrees on the access case. The rule on deleted functions forbids any reference to one. The reference does not say that the selection of the deallocation function is such a reference, and its rules of use do not name that function.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
