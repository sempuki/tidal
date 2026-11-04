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

Kept open on 2026-11-03 under decision 0054, because no principle on the page gives the answer. NS-P3 says that a using declaration conflicts with another declaration in the current scope, and it does not say whether a repeated using declaration of one entity conflicts. EDG 7.0 rejects the repeated declaration in a block.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. Two using declarations that name one declaration conflict only when both are in the same class scope. In a block, two using declarations that name the same entity refer to one entity and do not conflict. The older rule that rejected the block form is gone in C++23.
