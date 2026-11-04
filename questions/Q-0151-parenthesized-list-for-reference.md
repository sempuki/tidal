# Q-0151: Can a parenthesized list of several values initialize a reference to an aggregate?

**Status:** settled
**Affects:** REF-11, `tests/declarations/initialization/references/parenthesized-list-aggregate.cpp`

## Context

An aggregate can be initialized from a parenthesized list of values
(LIST-3). The compilers differ when the list initializes a reference to
an aggregate, as in `const Point& p(1, 2)`:

| Compiler | Result |
|---|---|
| Clang 18, Clang 22 | rejects ("reference cannot be initialized with multiple values") |
| GCC 14, GCC 16 | accepts, and binds the reference to a temporary `Point` |
| EDG 7.0 | rejects ("too many initializer values") |

The same split holds for a reference to an array, as in
`const int (&a)[2](1, 2)`. All compilers reject several values in
parentheses for a reference to a scalar.

## Evidence

`parenthesized-list-aggregate.cpp` and `reference-multiple-values.cpp`.

## Options

1. A parenthesized initializer of a reference holds one expression
   (Clang, EDG).
2. A parenthesized list makes a temporary aggregate, as a braced list
   does (GCC).

## Answer

No; Clang and EDG agree, GCC differs (option 1). Two of the three compilers reject the list, and REF-11 keeps
one rule for parentheses.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on overloading, references, and list initialization (`reports/review/2026-11-03-gap-review-questions-overloading.md`) supports this answer with no doubt, and REF-P1 gives the same answer: in parentheses, the initializer of a reference is one expression. GCC 16 still differs. Tidal records GCC 16's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for GCC 16.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on overloading, references, and list initialization (`reports/review/2026-11-03-gap-review-questions-overloading.md`) agrees with option 1. A parenthesized initializer for an entity that is not of class type must be a single expression, and a reference is not of class type. The same holds for a reference to an array. GCC's acceptance is a defect.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
