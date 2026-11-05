# Q-0082: Can new create an array of a class whose destructor is deleted?

**Status:** provisional
**Affects:** DTOR-3, `tests/classes/constructors/delete-array-deleted-dtor.cpp`

## Context

`new S[2]` where `S` has a deleted destructor. A new expression
for one object is valid on all compilers. For an array, Clang rejects the
expression. GCC and EDG accept it.

## Options

1. The array form is valid, as the single form is.
2. The array form needs a usable destructor, so the program is not valid.

## Provisional answer

No; the array form needs a usable destructor, so the program is not valid (option 2). Clang rejects the array form. GCC and EDG accept it.

Changed on 2026-11-03 to option 2. The review reads option 2, Clang 22 rejects, and CTOR-P2 gives the same answer: building several parts needs the destructor of each part, because the parts already built are destroyed when a later part fails, as for a default constructor whose member has a deleted destructor. DTOR-3 now lists an array `new`, and `delete-array-deleted-dtor.cpp` expects a rejection, with overrides for GCC and EDG. The question stays open under decision 0054, because two newest families differ from the answer.

Kept open on 2026-11-04 under decision 0054, because two newest families, GCC 16 and EDG 7.0, differ from the answer.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) reads option 2. An array `new` must destroy the elements that it has already built when a later element's initialization fails, so the destructor is needed, and a deleted or inaccessible destructor makes the program not valid, with a diagnostic. The single-object form needs no destructor and stays valid. Clang follows this reading. GCC and EDG accept.
