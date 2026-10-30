# Copy and move

A copy constructor and a copy assignment operator make an object equal to
another object of its class. A move constructor and a move assignment
operator do the same, and they can take resources from an object that a
program no longer needs. This page covers how to declare them, when a
class gets them implicitly, when they are deleted, what the implicit ones
do, and when a copy or move does not occur.

Depends on: constructors and destructors, reference binding, overload
resolution *(referenced)*, deleted and defaulted functions.
