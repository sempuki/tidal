# Q-0054: Is a second destruction rejected during constant evaluation?

**Status:** provisional
**Affects:** DTOR-7, `tests/classes/constructors/double-destroy-constant.cpp`

## Context

A function calls the destructor of an automatic object explicitly. The
object is then destroyed again at the end of its block. During constant
evaluation, Clang and GCC reject the second destruction. EDG 7.0 accepts
it.

## Options

1. The behavior is undefined, so constant evaluation rejects it.
2. Constant evaluation accepts it.

## Provisional answer

Yes; the behavior is undefined, so constant evaluation rejects it (option 1). Clang and GCC agree. A use of the object after the first
destruction is rejected by all compilers.

## Opinions
