# Q-0038: May a default argument name a parameter in an operand that is not evaluated?

**Status:** settled
**Affects:** DEFARG-4, `tests/declarations/functions/default-arguments/parameter-in-sizeof.cpp`

## Context

`int f(int a, int b = sizeof(a));` names the parameter `a` inside `sizeof`, which doesn't evaluate `a`. Clang 22 and 18 and GCC 16 and 14 accept the declaration. EDG 7.0 rejects it ("a parameter is not allowed"). A gap review summary also says the declaration is valid.

## Options

1. Valid: the ban covers only evaluated uses of a parameter.
2. Rejected: a default argument must not name a parameter at all.

## Answer

Valid; the ban covers only evaluated uses of a parameter (option 1). Two of three implementations accept the declaration, and the review summary agrees.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page gives the answer. DEFARG-P1 says that a default argument cannot name the other parameters, and it does not state the exception for an operand that is not evaluated. EDG 7.0 rejects the declaration.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). DEFARG-P1 fails step 3. DEFARG-P1 says that a default argument cannot name another parameter, so option 2 fits the model and option 1 does not. Item PQ widened DEFARG-P1: a parameter or a local also has a type that the declaration knows, so an operand that is not evaluated can name it, and `this` stays banned also there. The new test `tests/declarations/functions/default-arguments/this-in-sizeof-default.cpp` checks the `this` case, which every compiler rejects in a probe. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses DEFARG-P1.

Used on 2026-11-04 by the source-reader gap review of the text phase 8 wrote on the declarations and statements chapters (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`), under decision 0056. The review applied DEFARG-P1 to the program of `parameter-in-sizeof.cpp` without the rule text and reached the provisional answer: the operand of `sizeof` is not evaluated and needs only the parameter's type, so the declaration is valid.

Settled on 2026-11-04 under decision 0054. The review supports the answer with no doubt, and DEFARG-P1 gives the same answer by the test of the style guide (section 3), as the review's use shows. EDG 7.0 still differs, and the tests keep its overrides.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. A default argument must not use a parameter where the parameter would be potentially evaluated. The operand of `sizeof` is not evaluated, so the use is valid. EDG's rejection is a defect.

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on the declarations and statements chapters (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`) agrees with option 1. The reference bans every use of a parameter in a default argument that could be evaluated, and the operand of `sizeof` is not evaluated. The review also reads the reference as allowing a local constant of an enclosing function whose value is read, which DEFARG-P1 does not allow. That case is Q-0389 and does not touch this program.
