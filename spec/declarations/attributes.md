# Standard attributes

An attribute, written `[[name]]` or `[[name(arguments)]]`, attaches extra
information to a declaration, a statement, or a type. Tidal covers the
standard attributes: `noreturn`, `deprecated`, `fallthrough`,
`maybe_unused`, `nodiscard`, `likely`, `unlikely`, `no_unique_address`,
`assume`, and `carries_dependency`.

Most standard attributes don't change what a valid program does. They let
an implementation warn, or optimize, and the rules for them describe
*recommended practice*: behavior an implementation should provide but
need not (decision 0015). Rules are prefixed `ATTR`.

Depends on: declarations *(referenced)*, statements *(referenced)*,
classes *(referenced)*, [constant evaluation](../glossary.md#constant-evaluation).

Inputs: clang-sema-declattr, clang-sema-stmtattr, edg-attribute,
clang-diag-sema (see `inputs/MANIFEST.md`).
