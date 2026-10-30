# List initialization and aggregates

Initialization with a braced list, as in `int x{1};` or `S s = {1, 2};`.
Braces forbid narrowing conversions, fill arrays and aggregates member by
member, and allow designated initializers. Character arrays may be
initialized from string literals.

An *aggregate* here is an array, or a class with only public non-static data
members and no user-declared constructors *(classes referenced)*. Rules are
prefixed `LIST`.

Depends on: [declarators](../declarators.md), [storage duration](../storage-linkage.md),
[value category](../../glossary.md#value-category), conversions *(referenced)*,
classes *(referenced)*, [constant evaluation](../../glossary.md#constant-evaluation).

Inputs: clang-sema-init, gcc-cp-typeck2, gcc-cp-init, edg-decl-inits,
clang-diag-sema (see `inputs/MANIFEST.md`).
