# Q-0329: Does the initializer of a local `const` reference that binds a temporary run during constant evaluation?

**Status:** provisional
**Affects:** IF-25, IF-18, `tests/statements/if/consteval-local-reference.cpp`

## Context

IF-25 lists the places where an evaluation is a constant evaluation, and
so where the then branch of `if consteval` executes. The list includes
the constant initializer of a `const` local variable of integer or
enumeration type. CE-3 also makes a reference variable with a constant
initializer usable in constant expressions.

A local reference such as `const int& r = where();` binds a temporary
that the reference extends. The temporary is a local object. CE-P2 says
that the result of a constant evaluation cannot point to a local or
temporary object, so such an initializer is not a constant expression.

## Evidence

`consteval-local-reference.cpp` calls a `constexpr` function whose
`if consteval` gives 1 in its then branch and 2 in its else branch.
Clang 18 and Clang 22 give 2. GCC 14 and GCC 16 give 1. EDG translates
the program and does not run it. Both compiler families give 1 for a
local `const int` and 2 for a local `const double`
(`consteval-local-initializers.cpp`).

## Options

1. The initializer runs at run time, because the reference binds a
   local temporary and is not a constant initializer. `r` is 2.
2. The implementation tries the initializer as a constant, as for a
   `const` integer variable. `r` is 1.

## Provisional answer

The initializer runs at run time and `r` is 2 (option 1).
Option 1 follows CE-P2: a constant evaluation cannot give a reference
to a local temporary. Clang behaves this way, and GCC differs.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
