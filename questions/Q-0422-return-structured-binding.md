# Q-0422: Is a structured binding named in a return statement an xvalue?

**Status:** provisional
**Affects:** JMP-10, RET-6, `tests/statements/jumps/return-structured-binding-lvalue.cpp`
**Source:** point RV5-A-16 of the gap review of the text phase 12 wrote on classes, declarations, and foundations (`reports/review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md`), with a probe that a clean-room writer ran for it.
**Waits for:** a first review

## Context

A function with the return type `decltype(auto)` declares a structured
binding `auto [a, b]` of a local class object and returns `(a)`. JMP-10
makes the name of a local variable an xvalue in a `return` statement.
The review reads a structured binding as no such variable, so its name
stays an lvalue.

| Implementation | `decltype` of the call |
|---|---|
| Clang 18 and 22 | `int&` |
| GCC 14 and 16 | `int&` |
| EDG 7.0 | another type: the `static_assert` on `int&` fails |

## Evidence

The test in **Affects**, with an override for EDG.

## Options

1. A structured binding is not a variable that JMP-10 counts, and its
   name stays an lvalue (Clang, GCC, the review).
2. A structured binding of a local object counts, and its name is an
   xvalue (EDG).

## Provisional answer

The name stays an lvalue (option 1). Two of three families agree, and
the review reads the C++23 text this way.

## Opinions
