# Deleted and defaulted functions

`= delete` as a function's body makes any use of the function an error.
`= default` asks for the implicit definition of a special member function
*(classes referenced)*. This part covers both as they apply to functions
outside classes. Rules are prefixed `DEL`.

Depends on: [declarators](../declarators.md), [placeholder types](../auto.md),
[initialization](../initialization/forms.md), statements *(referenced)*,
overload resolution *(referenced)*, classes *(referenced)*.

Inputs: clang-sema-decl, clang-sema-declcxx, clang-sema-stmt,
clang-diag-sema, gcc-cp-decl, gcc-cp-typeck, edg-func-def (see
`inputs/MANIFEST.md`).
