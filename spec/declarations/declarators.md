# Declarators and cv-qualifiers

A declaration names a base type (`int`, a class, a placeholder) and then
one or more *declarators*. Each declarator gives a name and wraps the base
type in pointers, references, arrays, and functions. The *cv-qualifiers*
`const` and `volatile` may be applied at each level.

This section covers how declarators build types, the limits on which
combinations are allowed, and what `const` and `volatile` mean for objects.
Pointers to members and member function qualifiers belong to classes
*(referenced)*. Default arguments belong to function definitions.

Rules prefixed `DCL` cover declarators. Rules prefixed `CV` cover
cv-qualifiers.

Depends on: types *(referenced)*, [placeholder types](auto.md),
initialization *(referenced)*, [constant evaluation](../glossary.md#constant-evaluation),
classes *(referenced)*.

Inputs: clang-sema-type, clang-sema-decl, gcc-cp-decl, edg-decls,
clang-diag-sema (see `inputs/MANIFEST.md`).
