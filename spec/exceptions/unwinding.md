# Unwinding and termination

When an exception leaves a block, the objects of that block end in the
reverse of the order in which they began, as they would on any other
exit. This page covers that unwinding, the parts of an object that
exist when a constructor throws, the storage that a `new` expression
obtained, and the cases in which the program ends instead of unwinding:
an exception that leaves a `noexcept` function, a destructor during
unwinding, or `main`, and an exception with no handler.

Depends on: objects and lifetime, constructors and destructors,
exception specifications, new and delete.
