# Q-0026: May a range-based for variable have a storage class?

**Status:** provisional
**Affects:** RFOR-5, `tests/statements/range-for/storage-class.cpp`, `tests/statements/range-for/static-thread-local-declaration.cpp`

## Context

`for (static int v : a)` and `for (thread_local int v : a)` are rejected by
Clang 22 and 18 ("loop variable may not be declared 'static'"), GCC 16
("for-range-declaration cannot be 'static'"), and EDG 7.0. GCC 14.2 accepts
both.

## Options

1. Rejected. GCC 14's acceptance is a defect fixed in a later GCC.
2. Accepted.

## Provisional answer

Option 1. The newest version of every implementation in the matrix
rejects it.

## Opinions
