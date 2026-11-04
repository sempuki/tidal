# Q-0122: May a range-based `for` have an empty init statement?

**Status:** provisional
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

## Provisional answer

Yes; GCC rejects it (option 1). Two of three implementations accept it, and one `init_stmt`
for every statement that takes one keeps the grammar simple.

Kept open on 2026-11-03 under decision 0054, because no principle on the page states the case. RFOR-P1 says that the loop runs the initializer statement if there is one, and does not say whether an empty one is allowed. GCC 16 is the one newest family that differs.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) agrees with option 1. The range-based `for` uses the same init-statement production as the selection statements, and that production includes an expression statement with no expression. GCC's rejection is a defect.
