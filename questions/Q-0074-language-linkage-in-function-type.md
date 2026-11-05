# Q-0074: Is language linkage part of a function type?

**Status:** provisional
**Affects:** LSP-6, `tests/declarations/linkage-specifications/c-function-pointer-assign.cpp`, `tests/declarations/linkage-specifications/function-type-linkage-overload.cpp`

## Context

A function declared `extern "C"` and a function with C++ linkage can have
the same parameters and return type. EDG 7.0 treats their types as
different, so a pointer to the C function does not initialize a pointer
to a C++ function type. Clang and GCC treat the types as the same. All
three accept two declarations that overload on the two types.

EDG also gives C language linkage to a function type in a parameter and
in a type alias inside an `extern "C"` block, and rejects a C++
function as the argument for either
(`tests/declarations/linkage-specifications/c-parameter-type-linkage.cpp`,
`tests/declarations/linkage-specifications/c-callback-alias.cpp`).
Clang and GCC accept both programs.

## Options

1. Each implementation decides whether a function type includes its
   language linkage (implementation-defined).
2. Language linkage is part of the function type.
3. Language linkage is not part of the function type.

## Provisional answer

Implementation-defined (option 1). Two implementations ignore language linkage in the type and
one keeps it, and programs need to work with both.

Kept on 2026-11-03. The review reads option 2, which only EDG 7.0 follows. Clang 22 and GCC 16 treat the two types as one, and LSP-P3 gives the answer: the implementation chooses whether a function's type records its language linkage, and a portable program treats a pointer to a C function as a type of its own. Tidal keeps option 1 until the maintainer decides. The question is in row 4 of `documentation/decisions/pending.md`, and it stays open under decision 0054, because the review reads against the answer.

Kept open on 2026-11-04 under decision 0054, because the question waits on the maintainer (`documentation/decisions/pending.md`, row 4).

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) reads option 2. A function type with C language linkage and one with C++ language linkage are different types, even with the same parameters and return type, and nothing lets an implementation merge them. EDG follows this reading. Clang and GCC do not, and the review knows of no later change that removes the linkage from the type.
