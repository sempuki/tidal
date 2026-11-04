# Q-0264: Does an implementation diagnose an atomic constraint whose result changes between two checks?

**Status:** provisional
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

## Options

1. The program is not valid, and no diagnostic is required. Both
   results are allowed (CNS-10).
2. A diagnostic is required (GCC 16).

## Provisional answer

No diagnostic required; GCC 16 diagnoses (option 1). Finding the change needs a record of every earlier check, which most implementations do not keep.

Kept open on 2026-11-03 under decision 0054, because no principle on the page gives the answer. CNS-10 is an exception of CNS-P2, and no principle states that a result must stay the same.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). CNS-P2 failed the test, because CNS-P2 named CNS-10 in its `:exceptions:` line. Item PQ widened CNS-P2. A constraint means the same in every use, so an atomic constraint with the same arguments must give the same result at every check, and an implementation need not diagnose a change, because finding it would need a record of every earlier check. CNS-10 moved from the `:exceptions:` line to the `:rules:` line. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses CNS-P2.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) agrees with option 1. A program is not valid when two checks of the same atomic constraint with the same arguments give different results at different points, and no diagnostic is required. GCC 16's error and the other compilers' silence both conform.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
