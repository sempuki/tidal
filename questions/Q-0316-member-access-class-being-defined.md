# Q-0316: Can a member access name an earlier member of the class whose definition contains the access?

**Status:** settled
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

## Answer

Yes (PFX-11), as GCC and EDG do; Clang rejects (option 1). Two of three compilers accept the access, and option 1 gives
member access the same view of the class as unqualified lookup. The test
expects an acceptance and records Clang's result with an override.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page gives the answer. PFX-P3 says what a member access names, and no principle says whether a member access can see into a class that is being defined. Clang 22 is the only newest family that differs.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). PFX-P3 failed step 3, because no principle said whether a member access can see into a class that is being defined. Item PQ widened PFX-P3: inside the definition, a member access sees the members known at that place, as an unqualified name does, so a member declaration sees the earlier members and a member function body sees every member. The new test `tests/expressions/postfix/member-access-later-member-in-function-body.cpp` checks the function body case, which no PFX-11 test checked. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses PFX-P3.

Used on 2026-11-04 by the source-reader gap review of the text phase 8 wrote on expressions, foundations, and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`), Part B. The review applied PFX-P3 to the question's program, a member access to an earlier member inside the definition of its class (`member-access-class-being-defined.cpp`), without the rule text: a member declaration sees the members declared before it, so the access is accepted, which is the provisional answer.

Kept open on 2026-11-04 under decision 0055, because PFX-P3 and PFX-11 changed in this commit. The review named the other places where the class is complete, those of nested classes included, and PFX-P3 now lists them. The question waits for the next use of PFX-P3. Clang 22 is the one newest family that differs.

Used on 2026-11-04 by the principle-use batch PU1 (`reports/clean-room/PU1/`), under decision 0056. A clean-room reader applied PFX-P3, without the rule text, to the programs of `member-access-later-member-in-body.cpp`, `member-access-class-being-defined.cpp`, and `member-access-later-member-in-function-body.cpp`. Each answer names PFX-P3 and gives the outcome that the tests expect, so the use supports the answer.

Settled on 2026-11-04 under decision 0054. The use in batch PU1 supports the answer, the gap review of 2026-11-04 supports it with no doubt, and PFX-P3 gives the same answer by the test of the style guide (section 3). Clang 22 still differs, and the test keeps its override.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. A member access needs a complete class, except inside the definition of that class. The program is not valid if the result would differ once the class is complete. A member declared after the access is not found there. Clang's rejection is a defect.

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on expressions, foundations, and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`) agrees with option 1. A member access can name a member of a class that is not yet complete when the access is in that class's definition. The program is then not valid, with no diagnostic required, if the complete class would give another lookup result. The places where the class is complete include a function body, a default argument, a default template argument, an exception specification, and a default member initializer, also in nested classes.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
