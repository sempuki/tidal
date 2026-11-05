# Q-0122: May a range-based `for` have an empty init statement?

**Status:** settled
**Affects:** RFOR-3, the `range_for_stmt` production, `tests/statements/range-for/empty-init-statement.cpp`

## Context

An `if` and a plain `for` accept an empty init statement, a lone `;`.
For a range-based `for`, `for (; int x : v)` is accepted by Clang and
EDG and rejected by GCC 14 and 16, which read the `:` as part of a
qualified name.

## Evidence

`tests/RESULTS.md`, row `empty-init-statement.cpp`.

## Options

1. The range-based `for` takes the same `init_stmt` as `if` and `for`,
   including the empty one. GCC differs.
2. The range-based `for` excludes the empty init statement. Clang and
   EDG differ, and the grammar needs a second init production.

## Answer

Yes; GCC rejects it (option 1). Two of three implementations accept it, and one `init_stmt`
for every statement that takes one keeps the grammar simple.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page states the case. RFOR-P1 says that the loop runs the initializer statement if there is one, and does not say whether an empty one is allowed. GCC 16 is the one newest family that differs.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). RFOR-P1 failed step 3, because RFOR-P1 did not say which forms the initializer statement takes. Item PQ widened RFOR-P1: the initializer statement is the `init_stmt` of `if`, `switch`, and the plain `for`, so a lone `;` is valid in the range-based `for` too. RFOR-3 and its test `empty-init-statement.cpp` are already among the evidence of RFOR-P1. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses RFOR-P1.

Used on 2026-11-04 by the source-reader gap review of the text phase 8 wrote on the declarations and statements chapters (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`), under decision 0056. The review applied RFOR-P1 to the program of `empty-init-statement.cpp` without the rule text and reached the provisional answer: a lone semicolon is a form of the initializer statement that the four statements share, so the loop is valid.

Settled on 2026-11-04 under decision 0054. The review supports the answer with no doubt, and RFOR-P1 gives the same answer by the test of the style guide (section 3), as the review's use shows. GCC 16 still differs, and the tests keep its overrides.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) agrees with option 1. The range-based `for` uses the same init-statement production as the selection statements, and that production includes an expression statement with no expression. GCC's rejection is a defect.

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on the declarations and statements chapters (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`) agrees with option 1.
