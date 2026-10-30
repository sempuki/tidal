# Storage duration and linkage

Every variable has a *storage duration*, which decides when its storage
exists, and most names have a *linkage*, which decides whether the same name
in different scopes or translation units refers to the same entity. The
storage class specifiers `static`, `extern`, and `thread_local` set both,
together with where the declaration appears.

This section covers variables and functions declared at namespace scope and
block scope. Class members (`static` members, `mutable`), dynamic storage
(`new` and `delete`), threads, modules, and language linkage (`extern "C"`)
are *(referenced)*.

Rules prefixed `STO` cover storage duration and the specifiers. Rules
prefixed `LNK` cover linkage.

Depends on: declarations *(referenced)*, namespaces *(referenced)*,
initialization *(referenced)*, object lifetime *(referenced)*, the one
definition rule *(referenced)*.

Inputs: clang-sema-decl, gcc-cp-decl, edg-decls, clang-diag-sema (see
`inputs/MANIFEST.md`).
