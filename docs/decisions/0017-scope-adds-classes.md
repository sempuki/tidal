# 0017. Scope adds classes

Date: 2026-10-30
Status: Accepted. Extends 0002 and 0013.

## Context

Declarations, statements, and expressions are drafted. Many of their rules
mark classes *(referenced)*: constructors in initialization, members in
structured bindings, special members in `= default`, and class types in
`switch` conditions. Classes are the next largest gap. They are larger than
declarations, so they need an order of work.

## Decision

Classes are in scope, as listed in `docs/charter.md`. Sections are written
in this order, each through the full pipeline:

1. Class definitions and members (data members, member functions, `this`,
   static members, nested types, cv- and ref-qualified member functions)
2. Access control and friends
3. Constructors and destructors (member initializers, delegating and
   `explicit` constructors, order of construction and destruction)
4. Copy and move (implicit special members, copy elision)
5. Derived classes (bases, base initialization, name hiding, ambiguity)
6. Virtual functions and abstract classes (`override`, `final`, pure
   virtual functions, calls during construction)
7. Unions and bit-fields
8. Conversion functions
9. Operator overloading
10. Defaulted comparisons (`==` and `<=>`)

## Consequences

- Templates, overload resolution, and name lookup stay referenced. Class
  rules that depend on them say so.
- Tests for classes need no library headers, as before. Tests that need
  `std::strong_ordering` declare the minimum they need, as earlier tests
  do for `std::initializer_list`.
- Rules that mark classes *(referenced)* in earlier sections link to the
  class rules once those exist.
