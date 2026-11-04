# Q-0011: Is using an entity with no definition `no-diagnostic` or `reject`?

**Status:** settled
**Affects:** LNK-7, `tests/declarations/storage-linkage/missing-definition.cpp`

## Context

A program that calls a function declared in every translation unit but
defined in none compiles unit by unit without any diagnostic. Linking fails
with "undefined reference to" on the matrix compilers' linkers. The
compilers can't see the problem, because no single unit shows that the
definition is missing.

## Evidence

`tests/RESULTS.md`, rows `missing-definition.cpp` (link-fail on every
compiler that links) and `unused-missing-definition.cpp` (runs, because
an unused declaration needs no definition).

## Options

1. `no-diagnostic`. The rule can be broken without any implementation being
   required to report it, as with LNK-6.
2. `reject`, with the linker counted as part of the implementation. Every
   linker in the matrix reports the error.

## Answer

No-diagnostic (option 1). Whether a use needs a definition can depend on what the
optimizer removes, so a link failure is likely but not guaranteed. A test
that shows an unreported case would settle the question.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) supports this answer with no doubt, and STO-P4 gives the same answer: a unit is translated alone, so a missing definition is found when the program is linked, if at all. No newest family differs from the answer.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. A definition is required for each function or variable that is not inline and that the program uses, and a break of the rule needs no diagnostic. The review notes that the source text decides whether a use counts, so the optimizer is not the reason for the answer. The classification is the same.
