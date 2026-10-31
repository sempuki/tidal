# Exception specifications

A `noexcept` specifier states that a function does not let an exception
leave it. It is part of the function type. This page covers the forms of
the specifier, its place in function and pointer types, overriding, and
the functions that are `noexcept` without the keyword.

What happens when an exception does leave a `noexcept` function belongs
with exception handling *(referenced)*. DCL-14 and CONV-10 give the rules
for redeclarations and pointer conversions.

Depends on: declarators, standard conversions, virtual functions,
constructors and destructors, constant expressions.
