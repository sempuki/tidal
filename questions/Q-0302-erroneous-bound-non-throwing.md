# Q-0302: Does an erroneous array bound give a null pointer when the allocation function is non-throwing?

**Status:** settled
**Affects:** NEW-36, `tests/expressions/new-delete/bad-bound-nothrow.cpp`

## Context

`new (std::nothrow) int[n]` uses a non-throwing allocation function.
With a negative `n`, a size too large for the type of `sizeof`, or
fewer elements than the initializer has, the implementations differ:

- Clang 18 and 22 give a null pointer. Clang calls the allocation
  function with a size that it cannot meet, and the function returns
  null.
- GCC 14 and 16 throw `std::bad_array_new_length`, as they do for a
  potentially throwing function.

The checked source-reader inventory of 2026-10-31 says that the
expression yields a null pointer when the allocation function cannot
throw.

## Evidence

`tests/expressions/new-delete/bad-bound-nothrow.cpp`. A handler for
any exception distinguishes the two results.

## Options

1. A null pointer (NEW-36). GCC differs.
2. An exception, as for a potentially throwing function. Clang differs.

## Answer

Yes (NEW-36); Clang agrees, GCC throws (option 1). A program that uses a non-throwing allocation function
expects to test for a null result and expects no exception.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) supports this answer with no doubt, and NEW-P6 gives the same answer: with a non-throwing allocation function, an erroneous array bound makes the expression give null, and no allocation happens. GCC 16 still differs. Tidal records GCC 16's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for GCC 16.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. When the allocation function chosen for the expression cannot throw, an erroneous bound makes the result a null pointer of the result type, and no allocation function is called. GCC's exception is a defect.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
