# Q-0023: Is `int main() = delete;` rejected?

**Status:** compiler defect
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

## Provisional answer

Option 1. Every implementation's newest version in the matrix rejects it.

## Opinions
