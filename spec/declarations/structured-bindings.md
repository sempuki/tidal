# Structured bindings

A structured binding declaration introduces several names at once, each
referring to one element of an object: an array element, a data member of
a class, or an element obtained through the tuple protocol.

    auto [x, y] = point;

The declaration creates one hidden variable initialized from the
initializer, then makes each name refer to one element of it. Rules are
prefixed `SB`.

Depends on: [placeholder types](auto.md), [initialization](initialization/forms.md),
[reference binding](initialization/references.md), classes *(referenced)*,
templates *(referenced)*, `std::tuple_size` and `std::tuple_element`
*(library, referenced)*.

Inputs: clang-sema-declcxx, gcc-cp-decl, edg-decls, clang-diag-sema (see
`inputs/MANIFEST.md`).
