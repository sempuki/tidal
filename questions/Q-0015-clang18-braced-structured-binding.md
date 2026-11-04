# Q-0015: Is `auto [a, b]{arr};` valid when `arr` is an array?

**Status:** settled
**Affects:** SB-7, `tests/declarations/structured-bindings/initializer-braces.cpp`

## Context

A structured binding may be initialized with `=`, parentheses, or braces.
With an array initializer in braces, Clang 18.1.3 rejects the declaration
("cannot initialize an array element of type 'int' with an lvalue of type
'int[2]'"). Clang 22.1.8, GCC 16 and 14, and EDG 7.0 accept it, and all
compilers accept the `=` and parenthesized forms.

## Evidence

`tests/RESULTS.md`, rows `initializer-equals.cpp`,
`initializer-parens.cpp`, and `initializer-braces.cpp`.

## Options

1. Valid. Clang 18's rejection is a defect fixed in a later Clang.
2. Implementation-defined.

## Answer

Yes; Clang 18's rejection is fixed in later versions (option 1). The newer version of the same implementation and both other
implementations accept the braced form.

Settled on 2026-11-01 under decision 0051, because the newest version of each baseline family (Clang 22, GCC 16, EDG 7.0) agrees with this answer. Clang 18 still rejects the braced form, and the test keeps its override for Clang 18.

## Opinions

### Gap review, 2026-11-01

The source-reader gap review of question cluster 1 (`reports/review/2026-11-01-gap-review-question-cluster-1.md`) supports the answer. A structured binding with a braced initializer and an array initializes each element from the matching array element, and braces act as parentheses do.
