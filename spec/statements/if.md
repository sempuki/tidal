# The if statement

An `if` statement runs one of up to two statements depending on a condition.
Tidal covers three forms:

- the ordinary form, which tests a condition at run time,
- `if constexpr`, which tests a condition during translation and discards the
  branch not taken,
- `if consteval` and `if !consteval`, which test whether the statement is
  being evaluated as part of constant evaluation.

The ordinary form and `if constexpr` may begin with an initializer statement
that runs first and whose names are visible in the rest of the statement.

Depends on: expression statements, declarations *(referenced)*, scopes
*(referenced)*, contextual conversion to `bool`, constant evaluation,
templates *(referenced)*.

Inputs: clang-parse-stmt, clang-sema-stmt, clang-diag-sema,
clang-diag-parse, gcc-cp-parser, gcc-cp-semantics (see `inputs/MANIFEST.md`).
