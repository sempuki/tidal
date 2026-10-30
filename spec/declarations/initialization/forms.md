# Forms of initialization

How a variable gets its first value. An initializer may be written after
`=`, in parentheses, or in braces, or left out. Without one, an object is
*default-initialized*. With empty parentheses or braces, it is
*value-initialized*. Objects with static or thread storage are
*zero-initialized* first (STO-3).

This part covers scalars and arrays. Initialization by constructors belongs
to classes *(referenced)*. Rules are prefixed `INIT`.

Depends on: [declarators](../declarators.md), [storage duration](../storage-linkage.md),
[value category](../../glossary.md#value-category), conversions *(referenced)*,
classes *(referenced)*, [constant evaluation](../../glossary.md#constant-evaluation).

Inputs: clang-sema-init, gcc-cp-typeck2, gcc-cp-init, edg-decl-inits,
clang-diag-sema (see `inputs/MANIFEST.md`).
