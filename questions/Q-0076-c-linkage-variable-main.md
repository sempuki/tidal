# Q-0076: Can main, or a variable named main, have C language linkage?

**Status:** provisional
**Affects:** LSP-2, LSP-4, `tests/declarations/linkage-specifications/c-linkage-main.cpp`, `tests/declarations/linkage-specifications/c-variable-named-main.cpp`

## Context

A program declares `extern "C" int main()`, or a namespace declares
`extern "C" int main;`. GCC 16, Clang 22, and EDG reject both. GCC 14
and Clang 18 accept both.

## Options

1. The program is not valid in both cases.
2. Both declarations are valid.

## Provisional answer

No; the program is not valid in both cases (option 1). The newer versions agree with EDG.

Kept open on 2026-11-03 under decision 0054, because no principle on the page gives the answer. LSP-P1 and LSP-P2 say what C language linkage does, and neither says that `main`, or a variable with the spelling `main`, cannot have it. Clang 22, GCC 16, and EDG 7.0 all reject both forms.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. A linkage specification on the function `main` is not allowed, and a program that gives C language linkage to anything with the name `main`, in any namespace, is not valid.
