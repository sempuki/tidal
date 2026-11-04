# Q-0038: May a default argument name a parameter in an operand that is not evaluated?

**Status:** provisional
**Affects:** DEFARG-4, `tests/declarations/functions/default-arguments/parameter-in-sizeof.cpp`

## Context

`int f(int a, int b = sizeof(a));` names the parameter `a` inside `sizeof`, which doesn't evaluate `a`. Clang 22 and 18 and GCC 16 and 14 accept the declaration. EDG 7.0 rejects it ("a parameter is not allowed"). A gap review summary also says the declaration is valid.

## Options

1. Valid: the ban covers only evaluated uses of a parameter.
2. Rejected: a default argument must not name a parameter at all.

## Provisional answer

Valid; the ban covers only evaluated uses of a parameter (option 1). Two of three implementations accept the declaration, and the review summary agrees.

Kept open on 2026-11-03 under decision 0054, because no principle on the page gives the answer. DEFARG-P1 says that a default argument cannot name the other parameters, and it does not state the exception for an operand that is not evaluated. EDG 7.0 rejects the declaration.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. A default argument must not use a parameter where the parameter would be potentially evaluated. The operand of `sizeof` is not evaluated, so the use is valid. EDG's rejection is a defect.
