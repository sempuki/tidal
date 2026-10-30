# Placeholder types in variable declarations

A variable declaration may use `auto` or `decltype(auto)` in place of a
type. The variable's type is then worked out from its initializer. `auto`
may be combined with `const`, `volatile`, `*`, `&`, and `&&` to shape the
deduced type.

This section covers variables only, including variables declared in
conditions and initializer statements. Placeholders in function return
types, parameters, and template parameters belong to function definitions
and templates *(referenced)*.

Depends on: declarators *(referenced)*, initialization *(referenced)*,
[value category](../glossary.md#value-category), template argument
deduction *(referenced)*, `std::initializer_list` *(library, referenced)*.

Inputs: clang-sema-deduction, clang-sema-decl, gcc-cp-pt, gcc-cp-decl,
edg-overload, edg-decls (see `inputs/MANIFEST.md`).
