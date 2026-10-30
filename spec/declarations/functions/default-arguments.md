# Default arguments

A parameter may have a default argument, used when a call leaves that
argument out. This part covers where default arguments may appear, what
they may refer to, and when they are evaluated. Rules are prefixed
`DEFARG`.

Depends on: [declarators](../declarators.md), [placeholder types](../auto.md),
[initialization](../initialization/forms.md), statements *(referenced)*,
overload resolution *(referenced)*, classes *(referenced)*.

Inputs: clang-sema-decl, clang-sema-declcxx, clang-sema-stmt,
clang-diag-sema, gcc-cp-decl, gcc-cp-typeck, edg-func-def (see
`inputs/MANIFEST.md`).
