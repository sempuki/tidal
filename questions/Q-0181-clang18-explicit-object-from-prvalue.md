# Q-0181: Does a class prvalue initialize a by-value explicit object parameter without a copy or move?

**Status:** provisional
**Affects:** CLS-13, `tests/classes/members/explicit-object-from-prvalue.cpp`

## Context

A member function takes its object by value, as `this Fixed self`. The
call is `Fixed{}.get()`, and `Fixed` has deleted copy and move
constructors. Clang 22, the host Clang, GCC 14, GCC 16, the host GCC, and
EDG accept the call, and the run destroys one object. Clang 18 rejects
the call ("call to deleted constructor").

## Evidence

The test above, on the eight baseline compilers.

## Options

1. The prvalue initializes the parameter directly, as any prvalue of the
   same type does (VC-4).
2. The call makes a temporary and copies or moves it into the parameter.

## Provisional answer

Yes; Clang 18's rejection is fixed in Clang 22 (option 1). Every other implementation agrees, and Clang 22 accepts the
call, so the Clang 18 rejection is a defect that a later version fixes.

Reopened on 2026-11-01. The source-reader gap review of question cluster 1 (`reports/review/2026-11-01-gap-review-question-cluster-1.md`) reads the C++23 text as giving option 2, a copy or move from a materialized temporary. It reports that the newest compilers follow a later change to the language, which gives option 1. The specification follows the matrix until the maintainer decides whether Tidal follows that later change (Q-0007).

Kept on 2026-11-03. The answer follows the later change, which every newest compiler follows, until the maintainer decides Q-0007. The question is in rows 4 and 7 of `documentation/decisions/pending.md`, and it stays open under decision 0054, because the review reads against the answer.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) repeats the reading of the cluster 1 review. In the C++23 text a class prvalue before a dot is materialized as a temporary, and the explicit object parameter is copied or moved from it, so the call is not valid with both constructors deleted (option 2). A later change lets the prvalue initialize the parameter directly (option 1). Every compiler in the table except Clang 18 follows that change. The choice belongs to Q-0007.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-11-01

The source-reader gap review of question cluster 1 (`reports/review/2026-11-01-gap-review-question-cluster-1.md`) finds that the answer contradicts the C++23 text. Member access with a dot needs a glvalue, so the prvalue is first materialized, and the by-value parameter is then initialized from the temporary as an xvalue. That needs the move constructor, and the call is ill-formed when the move constructor is deleted (option 2). A later core defect resolution removes the materialization, and the newest compilers follow it. The review asks the question to name that dependency.

### Gap review of cluster 2, 2026-11-03

The source-reader gap review of question cluster 2 (`reports/review/2026-11-03-gap-review-question-cluster-2.md`) names the question only as the model for another case. It asks the writer to confirm a later defect resolution by probe before an answer depends on it, as for this question.
