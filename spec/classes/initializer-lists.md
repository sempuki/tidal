# List initialization of classes

A braced list can initialize an object of a class that has constructors.
The list selects a constructor. A constructor whose first parameter is
`std::initializer_list<E>` has priority. This page covers how a braced
list selects a constructor, initializer-list constructors, the array
behind a `std::initializer_list`, and narrowing.

List initialization of aggregates and scalars is with initialization.

Depends on: list initialization and aggregates, constructors and
destructors, overload resolution *(referenced)*.
