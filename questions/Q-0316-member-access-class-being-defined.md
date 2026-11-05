# Q-0316: Can a member access name an earlier member of the class whose definition contains the access?

**Status:** provisional
**Affects:** PFX-11, `tests/expressions/postfix/member-access-class-being-defined.cpp`

## Context

In a member declaration of a class `Pair`, the expression
`static_cast<Pair*>(nullptr)->first` names the data member `first`,
which an earlier member declaration declares. `Pair` is incomplete at
that place (CLS-2).

| Compiler | Result |
|---|---|
| GCC 16 | accepts, and the type of the access is `int` |
| EDG 7.0 | accepts |
| Clang 22 | rejects: member access into incomplete type |

For a member declared after the access, every compiler rejects the
program. GCC and EDG say that the class has no such member, and Clang
says that the class is incomplete.

## Evidence

`tests/expressions/postfix/member-access-class-being-defined.cpp` and
`tests/expressions/postfix/member-access-later-member-in-body.cpp`.

## Options

1. The access is valid and sees the members declared before it, as an
   unqualified name in a member declaration does (NAM-6).
2. The access needs a complete class, as any other member access does.

## Provisional answer

Yes (PFX-11), as GCC and EDG do; Clang rejects (option 1). Two of three compilers accept the access, and option 1 gives
member access the same view of the class as unqualified lookup. The test
expects an acceptance and records Clang's result with an override.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page gives the answer. PFX-P3 says what a member access names, and no principle says whether a member access can see into a class that is being defined. Clang 22 is the only newest family that differs.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). PFX-P3 failed step 3, because no principle said whether a member access can see into a class that is being defined. Item PQ widened PFX-P3: inside the definition, a member access sees the members known at that place, as an unqualified name does, so a member declaration sees the earlier members and a member function body sees every member. The new test `tests/expressions/postfix/member-access-later-member-in-function-body.cpp` checks the function body case, which no PFX-11 test checked. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses PFX-P3.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. A member access needs a complete class, except inside the definition of that class. The program is not valid if the result would differ once the class is complete. A member declared after the access is not found there. Clang's rejection is a defect.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
