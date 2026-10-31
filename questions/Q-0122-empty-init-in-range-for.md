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

Option 1. Two of three implementations accept it, and one `init_stmt`
for every statement that takes one keeps the grammar simple.

## Opinions
