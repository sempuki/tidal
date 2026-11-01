# Q-0035: Is `delete` of a `void*` rejected?

**Status:** provisional
**Affects:** NEW-8, `tests/expressions/new-delete/delete-void-pointer.cpp`

## Context

GCC 16 and 14 accept `delete p;` where `p` is a `void*`, with a warning that the deletion is undefined. Clang 22 and 18 reject the deletion under `-pedantic-errors` (`-Wdelete-incomplete`). EDG 7.0 rejects it.

## Options

1. Rejected: `delete` needs a pointer to a complete object type.
2. Accepted, with undefined behavior at run time, as GCC treats it.

## Provisional answer

Yes (option 1). Two of three implementations reject the deletion, and GCC's own warning says the deletion can't work.

## Opinions
