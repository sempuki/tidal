# Q-0301: What does a `new` expression do with an erroneous run-time array bound?

**Status:** provisional
**Affects:** NEW-35, `tests/expressions/new-delete/bad-bound-negative.cpp`, `tests/expressions/new-delete/bad-bound-too-large.cpp`, `tests/expressions/new-delete/bad-bound-too-few.cpp`, `tests/foundations/language-support/exception-classes-array-length.cpp`

## Context

A run-time bound of `new T[n]` is erroneous if it is negative, if the
size of the array does not fit in the type of `sizeof`, or if the
initializer has more elements than the bound. The implementations
differ:

| Bound | GCC 14 and 16 | Clang 18 and 22 |
|---|---|---|
| negative | no allocation call, throws `std::bad_array_new_length` | calls the allocation function with a size it cannot meet; the function throws `std::bad_alloc` |
| size too large | as for a negative bound | as for a negative bound |
| fewer than the initializer's elements | calls the allocation function, frees the storage, throws `std::bad_array_new_length` | as for a negative bound |

The checked source-reader inventory of 2026-10-31 says that the
expression calls no allocation function and throws an exception that
a handler for `std::bad_array_new_length` catches. Constant evaluation
rejects each erroneous bound on every compiler.

## Evidence

The tests above. A replaced `operator new[]` counts its calls and
throws `std::bad_alloc` for a size that does not fit in a static
buffer. Under `-O2`, GCC 16 also makes no call for the third row, as
NEW-14 allows.

## Options

1. NEW-35 as the inventory describes. Clang differs in every row, and
   GCC differs in the third row.
2. The expression throws an exception that a handler for
   `std::bad_alloc` catches, and whether it calls an allocation
   function is unspecified. All compilers agree with this rule, but a
   handler for `std::bad_array_new_length` misses the Clang exception.

## Provisional answer

Option 1. A program can still catch the exception of every
implementation with a handler for `std::bad_alloc`, because
`std::bad_array_new_length` derives from it.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
