# Q-0302: Does an erroneous array bound give a null pointer when the allocation function is non-throwing?

**Status:** provisional
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

## Provisional answer

Option 1. A program that uses a non-throwing allocation function
expects to test for a null result and expects no exception.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
