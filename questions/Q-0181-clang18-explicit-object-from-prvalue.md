# Q-0181: Does a class prvalue initialize a by-value explicit object parameter without a copy or move?

**Status:** settled
**Affects:** CLS-13, `tests/classes/members/explicit-object-from-prvalue.cpp`

## Context

A member function takes its object by value, as `this Fixed self`. The
call is `Fixed{}.get()`, and `Fixed` has deleted copy and move
constructors. Clang 22, the host Clang, GCC 14, GCC 16, the host GCC, and
EDG accept the call, and the run destroys one object. Clang 18 rejects
the call ("call to deleted constructor").

## Evidence

The test above, on the eight baseline compilers.

## Options

1. The prvalue initializes the parameter directly, as any prvalue of the
   same type does (VC-4).
2. The call makes a temporary and copies or moves it into the parameter.

## Answer

Yes; Clang 18's rejection is fixed in Clang 22 (option 1). Every other implementation agrees, and Clang 22 accepts the
call, so the Clang 18 rejection is a defect that a later version fixes.

Settled on 2026-11-01 under decision 0051, because the newest version of each baseline family (Clang 22, GCC 16, EDG 7.0) agrees with this answer. Clang 18 still rejects the call, and the test keeps its override for Clang 18.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
