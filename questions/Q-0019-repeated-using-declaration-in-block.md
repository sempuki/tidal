# Q-0019: May a using declaration be repeated in a block?

**Status:** provisional
**Affects:** NS-7, `tests/declarations/namespaces/using-declaration-repeat-block.cpp`

## Context

`using N::x;` written twice at namespace scope is accepted by every
compiler. Written twice in a block, it is accepted by Clang 22 and 18 and
GCC 16, and rejected by GCC 14 and EDG 7.0 ("has already been declared in
the current scope"). GCC changed its behavior between versions 14 and 16.

## Evidence

`tests/RESULTS.md`, rows `using-declaration-repeat-namespace.cpp` and
`using-declaration-repeat-block.cpp`.

## Options

1. Allowed in blocks as at namespace scope. Matches the newest Clang and
   GCC.
2. Rejected in blocks. Matches GCC 14 and EDG.

## Provisional answer

Yes, as the newest Clang and GCC allow (option 1). Both Clang versions and the newer GCC accept the repeated
declaration. GCC's move from rejecting to accepting suggests that the
implementations are converging on acceptance.

## Opinions
