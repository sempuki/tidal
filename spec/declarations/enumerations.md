# Enumerations

An enumeration is a distinct integer-like type whose named values, its
*enumerators*, are listed in its definition. A *scoped* enumeration
(`enum class` or `enum struct`) keeps its enumerators inside its own scope
and doesn't convert implicitly to integers. An *unscoped* enumeration
(`enum`) puts its enumerators in the enclosing scope and converts
implicitly. Either kind may name a fixed *underlying type*, the integer
type that holds its values. Rules are prefixed `ENUM`.

Depends on: [usual arithmetic conversions](../glossary.md#usual-arithmetic-conversions),
[integer promotion](../glossary.md#integer-promotion),
[list initialization](initialization/list.md), name lookup *(referenced)*,
[constant evaluation](../glossary.md#constant-evaluation).

Inputs: clang-sema-decl, gcc-cp-decl, edg-decls, clang-diag-sema (see
`inputs/MANIFEST.md`).
