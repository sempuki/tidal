# Q-0264: Does an implementation diagnose an atomic constraint whose result changes between two checks?

**Status:** settled
**Affects:** CNS-10, `tests/templates/constraints/satisfaction-changes.cpp`

## Context

`Complete<Late>` is checked before and after `Late` is defined. The
requirement `sizeof(T)` fails the first time and is valid the second
time.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts, and gives false both times |
| GCC 14 | accepts, and gives false both times |
| GCC 16, host | rejects: satisfaction value of atomic constraint changed from false to true |
| EDG 7.0 | accepts, and gives false both times |

## Evidence

The test above.

For a function declared between two checks, every compiler keeps the first result, GCC 16 included (`satisfaction-changes-declaration.cpp`). For a concept-id whose requirement names a private member, checked inside a member of the class and at namespace scope, every compiler keeps the first result in either order, GCC 16 included (`requirement-access-concept-kept.cpp`, `requirement-access-concept-kept-false.cpp`).

## Options

1. The program is not valid, and no diagnostic is required. Both
   results are allowed (CNS-10).
2. A diagnostic is required (GCC 16).

## Answer

No diagnostic required; GCC 16 diagnoses (option 1). Finding the change needs a record of every earlier check, which most implementations do not keep.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page gives the answer. CNS-10 is an exception of CNS-P2, and no principle states that a result must stay the same.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). CNS-P2 failed the test, because CNS-P2 named CNS-10 in its `:exceptions:` line. Item PQ widened CNS-P2. A constraint means the same in every use, so an atomic constraint with the same arguments must give the same result at every check, and an implementation need not diagnose a change, because finding it would need a record of every earlier check. CNS-10 moved from the `:exceptions:` line to the `:rules:` line. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses CNS-P2.

Used on 2026-11-04 by the source-reader gap review of the text phase 8 wrote on expressions, foundations, and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`), Part B. The review applied CNS-P2 to the question's program, a constraint whose result changes between two checks (`satisfaction-changes.cpp`), without the rule text: a constraint means the same in every use, so a changed result is not valid, with no diagnostic required. That is the provisional answer, and the model rules out the other options.

Settled on 2026-11-04 under decision 0054. The review supports the answer with no doubt, and CNS-P2 gives the same answer under the test of the style guide (section 3). GCC 16 reports the change, and the other newest families do not. The class of CNS-10 allows both, so no newest family differs from the answer.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) agrees with option 1. A program is not valid when two checks of the same atomic constraint with the same arguments give different results at different points, and no diagnostic is required. GCC 16's error and the other compilers' silence both conform.

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on expressions, foundations, and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`) agrees with option 1. The reference makes a different result for identical atomic constraints and arguments at two points ill-formed, with no diagnostic required.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
