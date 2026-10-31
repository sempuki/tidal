# Defaulted comparisons

A class can default its equality and three-way comparison operators. A
defaulted comparison compares the bases and members of two objects in
order. An expression can also use a comparison operator that the
implementation rewrites in terms of `==` or `<=>`. This page covers
defaulted comparison functions, their return types, when they are
deleted, and rewritten and reversed comparisons.

Depends on: comparison operators, operator overloading, derived classes.
