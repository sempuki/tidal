# Q-0076: Can main, or a variable named main, have C language linkage?

**Status:** settled
**Affects:** LSP-2, LSP-4, `tests/declarations/linkage-specifications/c-linkage-main.cpp`, `tests/declarations/linkage-specifications/c-variable-named-main.cpp`

## Context

A program declares `extern "C" int main()`, or a namespace declares
`extern "C" int main;`. GCC 16, Clang 22, and EDG reject both. GCC 14
and Clang 18 accept both.

## Options

1. The program is not valid in both cases.
2. Both declarations are valid.

## Answer

No; the program is not valid in both cases (option 1). The newer versions agree with EDG.

Settled on 2026-11-04 under decision 0051, because the newest version of each baseline family (Clang 22, GCC 16, EDG 7.0) agrees with this answer. GCC 14 and Clang 18 accept both forms, and the tests keep their overrides for GCC 14 and Clang 18. The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages agrees with option 1. Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). LSP-P1 and LSP-P2 do not give the answer, and decision 0051 needs no principle.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. A linkage specification on the function `main` is not allowed, and a program that gives C language linkage to anything with the name `main`, in any namespace, is not valid.
