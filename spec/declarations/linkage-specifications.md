# Linkage specifications

A linkage specification gives declarations a *language linkage*, such as
`extern "C"`. A function with C language linkage can be called from code
in C, and its name is not mangled. This page covers the forms of a linkage
specification, where it can occur, and the rules for names with C
language linkage.

Depends on: storage duration and linkage, namespaces, function
definitions.
