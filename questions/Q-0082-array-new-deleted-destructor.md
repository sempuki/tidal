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

Yes; the array form is valid, as the single form is (option 1). GCC and EDG accept the array form.

## Opinions
