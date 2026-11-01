# Q-0130: Does EDG's acceptance of some uses of an enclosing function's constant change ODR-2?

**Status:** provisional
**Affects:** ODR-2, `tests/foundations/odr/mutable-member-read-in-lambda.cpp`, `tests/foundations/odr/local-class-binds-constant.cpp`

## Context

A lambda without captures, and a member function of a local class, can
read the value of a constant of the enclosing function. Some uses need
the object itself:

| Case | Clang 18, Clang 22 | GCC 14, GCC 16 | EDG 7.0 |
|---|---|---|---|
| A lambda reads a `mutable` member of a local `constexpr` object | rejects | rejects | accepts |
| A local class's member function binds a reference to a local constant | rejects | rejects | accepts |

EDG rejects the address of a local constant in a lambda
(`expressions/lambdas/constant-address-needs-capture.cpp`).

## Evidence

The two tests above, compiled with `--c++23 --strict` for EDG.

## Options

1. Keep ODR-2 as Clang and GCC behave. Both cases are uses, and the
   programs are not valid.
2. Treat both cases as reads of a value, as EDG does.

## Provisional answer

No; EDG differs (option 1). Clang and GCC agree, and a `mutable` member or a reference
needs the object. The tests expect EDG to accept the programs.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
