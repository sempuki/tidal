# Virtual functions and abstract classes

A virtual function is a member function that a derived class can
override. A call through a pointer or reference to a base calls the
override for the class that the object was created as. An abstract class
has a pure virtual function, and a program cannot create an object of it.
This page covers declaring and overriding virtual functions, `override`
and `final`, return types of overrides, calls during construction and
destruction, virtual destructors, and abstract classes.

Depends on: derived classes, constructors and destructors, access
control, `new` and `delete`.
