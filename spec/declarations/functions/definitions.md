# Function definitions

A function definition gives a function's body. This part covers the
parameters and their scope, what happens when control reaches the end of a
body, the predefined variable `__func__`, and the special rules for `main`.
Rules are prefixed `FN`.

Depends on: [declarators](../declarators.md), [placeholder types](../auto.md),
[initialization](../initialization/forms.md), statements *(referenced)*,
overload resolution *(referenced)*, classes *(referenced)*.

Inputs: clang-sema-decl, clang-sema-declcxx, clang-sema-stmt,
clang-diag-sema, gcc-cp-decl, gcc-cp-typeck, edg-func-def (see
`inputs/MANIFEST.md`).
