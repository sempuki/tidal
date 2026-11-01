# Q-0300: Does a `new` expression for a type of ordinary alignment fall back to an aligned allocation function?

**Status:** provisional
**Affects:** NEW-29, NEW-30, `tests/expressions/new-delete/aligned-form-only-for-ordinary-type.cpp`

## Context

For a type whose alignment exceeds the default new alignment, a `new`
expression passes the alignment. If no allocation function accepts it,
the expression tries again without the alignment (NEW-29). The checked
source-reader inventory of 2026-10-31 also describes the reverse step:
for a type of ordinary alignment, if no function accepts the size
alone, the expression tries again with the alignment added.

Clang 18 and 22, GCC 14 and 16, and EDG all reject a `new` expression
for a class of ordinary alignment whose only `operator new` takes an
alignment. None of them takes the reverse step.

## Evidence

`tests/expressions/new-delete/aligned-form-only-for-ordinary-type.cpp`.
Clang and GCC say "no matching function for call to 'operator new'".
EDG says the function "cannot be called".

## Options

1. No reverse step (NEW-30). The program is rejected.
2. A reverse step, as the inventory describes. No implementation in
   the matrix supports it in C++23 mode. The inventory may describe a
   later version of the language (Q-0007).

## Provisional answer

Option 1, the behavior of every implementation in the matrix.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
