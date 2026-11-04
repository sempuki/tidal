# Q-0130: Does EDG's acceptance of some uses of an enclosing function's constant change ODR-2?

**Status:** settled
**Affects:** ODR-2, `tests/foundations/odr/mutable-member-read-in-lambda.cpp`, `tests/foundations/odr/local-class-binds-constant.cpp`, `tests/foundations/odr/member-read-beside-mutable-in-lambda.cpp`

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

## Answer

No; EDG differs (option 1). Clang and GCC agree, and a `mutable` member or a reference
needs the object. The tests expect EDG to accept the programs.

Settled on 2026-11-03 under decision 0054. The review supports this answer with no doubt, and ODR-P1 gives the same answer: a read of a constant with a `mutable` member, and a reference binding, are uses. Clang 22 and GCC 16 follow the answer, and also reject a read of a member that is not `mutable` beside a `mutable` one (`member-read-beside-mutable-in-lambda.cpp`). EDG 7.0 still differs, and the tests keep their overrides for EDG.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-11-03

The source-reader gap review of program structure (`reports/review/2026-11-03-gap-review-program-structure.md`) reads the reference as supporting option 1, the provisional answer, with no doubt. The exception for a read of a constant does not apply to a variable with any `mutable` subobject, and a reference binding reads no value. Both programs use the variable where it cannot be used, so they are not valid.
