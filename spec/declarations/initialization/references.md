# Reference binding

How a reference is bound by its initializer: which expressions each kind of
reference accepts, when a temporary object is created for it, and how long
that temporary lives. Rules are prefixed `REF`.

Depends on: [declarators](../declarators.md), [storage duration](../storage-linkage.md),
[value category](../../glossary.md#value-category), conversions *(referenced)*,
classes *(referenced)*, [constant evaluation](../../glossary.md#constant-evaluation).

Inputs: clang-sema-init, gcc-cp-typeck2, gcc-cp-init, edg-decl-inits,
clang-diag-sema (see `inputs/MANIFEST.md`).
