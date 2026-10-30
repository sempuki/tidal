# Namespaces and using

A *namespace* is a named scope at namespace level that groups declarations.
Its members are named from outside as `N::member`. A *using declaration*
(`using N::name;`) declares one name from a namespace in the current scope.
A *using directive* (`using namespace N;`) makes every name in a namespace
visible to unqualified lookup in the current scope. Rules are prefixed `NS`.

Name lookup in general, argument-dependent lookup, and using declarations
inside classes belong to other topics *(referenced)*.

Depends on: scopes *(referenced)*, name lookup *(referenced)*,
[linkage](storage-linkage.md), overload resolution *(referenced)*.

Inputs: clang-sema-declcxx, clang-diag-sema, gcc-cp-decl, edg-decls (see
`inputs/MANIFEST.md`).
