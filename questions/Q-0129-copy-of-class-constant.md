# Q-0129: Is a copy of a whole class-type constant a use of the constant?

**Status:** provisional
**Affects:** ODR-2, ODR-3, `tests/foundations/odr/class-constant-copy-in-lambda.cpp`

## Context

A lambda without captures can read the value of a local constant of the
enclosing function, because the read is no use of the variable (ODR-3).
The read of a member of a `constexpr` object of class type is also no
use on every compiler. The compilers differ when the lambda copies the
whole object:

| Compiler | Lambda returns a copy of a local `constexpr` object of a class with one `int` member |
|---|---|
| Clang 18, Clang 22 | rejects: the variable cannot be captured implicitly |
| GCC 14, GCC 16 | accepts |
| EDG 7.0 | accepts |

The copy calls the class's copy constructor, and the constructor takes
the object by reference. Clang counts the reference as a use. GCC and
EDG read the object's value as a constant.

## Evidence

`class-constant-copy-in-lambda.cpp`. On Clang and GCC, a read of a
`mutable` member is a use (`mutable-member-read-in-lambda.cpp`).

## Options

1. The copy is no use. A lambda can copy a local constant of class type
   without capturing the constant.
2. The copy is a use. The lambda must capture the constant.

## Provisional answer

Option 1, because Tidal follows the majority where the compilers
disagree. The test expects Clang to reject the program.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
