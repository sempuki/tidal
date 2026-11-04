# Q-0129: Is a copy of a whole class-type constant a use of the constant?

**Status:** provisional
**Affects:** ODR-P1, ODR-2, ODR-3, `tests/foundations/odr/class-constant-copy-in-lambda.cpp`

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

Yes; the copy is a use, and the lambda must capture the constant (option 2). Clang rejects the program, and GCC and EDG accept it. The test expects a rejection, with overrides for GCC and EDG.

Changed on 2026-11-03. The source-reader gap review of program structure (`reports/review/2026-11-03-gap-review-program-structure.md`) reads the reference as supporting option 2. The reading matches Clang 22, and Tidal's own rules give the same answer. ODR-P1 counts an expression that binds a reference as a use, and the copy constructor binds its parameter to the constant. ODR-3 now exempts only a read of a scalar part of a constant with no `mutable` member, and the ODR-P1 table has a row for the copy.

Kept open on 2026-11-03 under decision 0054, because ODR-3 changed on the same day, and a question settles only after a reader or a review has used the changed rule (decision 0055). GCC 16 and EDG 7.0 also differ from the answer.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-11-03

The source-reader gap review of program structure (`reports/review/2026-11-03-gap-review-program-structure.md`) reads the reference as supporting option 2. The exception for a read of a constant covers only a read that converts a value of a type that is neither a class type nor volatile. A copy of a whole class object makes no such conversion, because the copy constructor takes the object by reference, and the binding needs the object itself. So the copy uses the variable, and in a lambda with no captures the program is not valid, as Clang says. The exception also needs a variable with no `mutable` subobject at all, so a read of a member that is not `mutable` is a use when another member is `mutable`. These limits entered the reference through a core defect resolution in the C++20 cycle, and GCC and EDG appear to keep the older rule.
