# Q-0377: Does the body of a class defined with a qualified name see the members of the qualifier without qualification?

**Status:** provisional
**Affects:** NAM-21, LSUP-3, `tests/foundations/names/definition-class-body-sees-namespace.cpp`, `tests/foundations/language-support/tuple-protocol-page-example.cpp`
**Source:** reports/clean-room/roadmap-5/programmer.md, section 1.4 (the `prelude.cpp` row) and section 1.5

## Context

NAM-21 lists the parts of a declaration that come after a qualified
name and see the members of the qualifier: the parameters, an array
bound, an exception specification, a trailing return type, the
function body, and the initializer. The list does not name the body of
a class that the declaration defines.

A class can be defined with a qualified name in two common forms. A
class declared in a namespace is defined outside the namespace, as in
`struct N::C { ... };`. An explicit or partial specialization of a
library template is defined outside namespace `std`, as in the tuple
protocol example of the language-support page, whose specialization
`std::tuple_size<Range>` uses `size_t` without qualification.

Every compiler in the matrix accepts both forms. The clean-room front
end rejects the tuple protocol example: it looks up `size_t` in the
class body and does not search `std`.

## Evidence

`tests/foundations/names/definition-class-body-sees-namespace.cpp` and
`tests/foundations/language-support/tuple-protocol-page-example.cpp`
run to result 0 on the eight baseline compilers and the two report
compilers.

## Options

1. The class body comes after the qualified name and sees the members
   of the qualifier, as the function body does. Every compiler behaves
   this way.
2. NAM-21 covers only functions and variables. A class body needs
   qualification. No compiler behaves this way, and the page example
   would be invalid.

## Provisional answer

Yes; the class body sees the members of the qualifier (option 1).
Every compiler agrees, and the language-support page's own example
depends on the answer. NAM-21 now names the class body.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
