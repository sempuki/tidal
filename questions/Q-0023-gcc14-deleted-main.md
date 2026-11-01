# Q-0023: Is `int main() = delete;` rejected?

**Status:** settled
**Affects:** DEL-2, `tests/declarations/functions/deleted/deleted-main.cpp`

## Context

Clang 22 and 18, GCC 16, and EDG 7.0 reject a deleted `main` ("'main' is
not allowed to be deleted", "'::main' cannot be deleted"). GCC 14.2 accepts
it.

## Evidence

`tests/RESULTS.md`, row `deleted-main.cpp`.

## Options

1. Rejected. GCC 14's acceptance is a defect fixed in a later GCC.
2. Accepted.

## Answer

Yes; GCC 14's acceptance is fixed in GCC 16 (option 1). Every implementation's newest version in the matrix rejects a
deleted `main`.

Settled on 2026-11-01 under decision 0051, because the newest version of each baseline family (Clang 22, GCC 16, EDG 7.0) agrees with this answer. GCC 14 still accepts a deleted `main`, and the test keeps its override for GCC 14.

## Opinions
