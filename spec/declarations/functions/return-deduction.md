# Return type deduction

A function declared with `auto` or `decltype(auto)` as its return type
gets its return type from its `return` statements. Rules are prefixed
`RET`.

Depends on: [declarators](../declarators.md), [placeholder types](../auto.md),
[initialization](../initialization/forms.md), statements *(referenced)*,
overload resolution *(referenced)*, classes *(referenced)*.

Inputs: clang-sema-decl, clang-sema-declcxx, clang-sema-stmt,
clang-diag-sema, gcc-cp-decl, gcc-cp-typeck, edg-func-def (see
`inputs/MANIFEST.md`).
