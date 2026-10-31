# Q-0125: Does Clang 18 omitting the sized deallocation functions change NEW-19?

**Status:** compiler defect
**Affects:** NEW-19, `tests/expressions/new-delete/global-functions-by-name.cpp`

## Context

NEW-19 says that every program has the global allocation and
deallocation functions, including the sized forms of `operator delete`
and `operator delete[]`, and can call them by name. Clang 22, GCC 14
and 16, and EDG accept a call of the sized forms with no declaration in
the program. Clang 18 rejects it, because it does not declare the sized
forms unless a flag asks for them.

## Evidence

`tests/RESULTS.md`, row `global-functions-by-name.cpp`.

## Options

1. Keep NEW-19 as written. Clang 18 differs, and Clang 22 agrees with
   the other compilers.
2. Leave the sized forms out of NEW-19.

## Provisional answer

Option 1. The newest version of every implementation in the matrix
declares the sized forms.

## Opinions
