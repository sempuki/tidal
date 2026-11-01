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

## Opinions
