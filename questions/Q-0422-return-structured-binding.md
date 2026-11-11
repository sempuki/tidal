# Q-0422: Is a structured binding named in a return statement an xvalue?

**Status:** provisional
**Affects:** JMP-10, RET-6, `tests/statements/jumps/return-structured-binding-lvalue.cpp`
**Source:** point RV5-A-16 of the gap review of the text phase 12 wrote on classes, declarations, and foundations (`reports/review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md`), with a probe that a clean-room writer ran for it.
**Waits for:** a principle (JMP-P3)

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
| EDG 7.0 | `int&&` |

EDG also gives `int&&` when the binding is declared `auto&` to a local
object, and Clang and GCC give `int&` there too. For `return a;` with
the return type `decltype(auto)`, every compiler gives `int`
(`reports/clean-room/roadmap-10/writer-probes/q-0422-binding-category.json`).

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

Kept open on 2026-11-10 under decision 0054 (item RV6). The review supports the answer with no doubt, and only EDG differs. No principle gives the answer. JMP-P3 names a local variable and a parameter, and a reader cannot tell from JMP-P3 whether a structured binding is a local variable. Both options fit JMP-P3, so JMP-P3 fails step 3 of the principle test.

## Opinions

### Gap review, 2026-11-10 (phase 13 text, classes, declarations, foundations, and statements)

The source-reader gap review of the text phase 13 wrote on classes, declarations, foundations, and statements (`reports/review/2026-11-10-gap-review-phase-13-classes-declarations-foundations.md`) reads the question: supports with no doubt (RV6-A-9). A structured binding is an entity of its own kind and is no variable. The rule that makes a name an xvalue in a `return` statement covers only variables with automatic storage, so the binding's name stays an lvalue. The same holds for a binding to the data members of a class and for the tuple-like form.
