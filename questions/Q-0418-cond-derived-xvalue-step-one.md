# Q-0418: In step 1 of COND-16, can an xvalue operand bind the lvalue reference to the other operand's type?

**Status:** provisional
**Affects:** COND-16, `tests/expressions/conditional/class-derived-xvalue-const-base-lvalue.cpp`, `tests/expressions/conditional/class-const-lvalue-against-xvalue.cpp`
**Source:** finding I4-2 of `reports/clean-room/I4/report.md`, from the probes that a clean-room writer ran for it.
**Waits for:** the maintainer (pending.md row 4)

## Context

Step 1 of COND-16 tries to convert an operand `X` to an lvalue of the
other operand's type `T` when the other operand is an lvalue. A
`const T&` can bind directly to an xvalue. For a `const A` lvalue
against an `A` xvalue, every baseline compiler gives a `const A`
prvalue, so step 1 does not take the xvalue. For a `Derived` xvalue
against a `const Base` lvalue the compilers split.

| Implementation | `Derived` xvalue against `const Base` lvalue |
|---|---|
| Clang 22 | a `const Base` lvalue |
| GCC 16 | a `const Base` prvalue |
| EDG 7.0 | a `const Base` prvalue |

## Evidence

The tests in **Affects**. The derived test has an override for Clang.

## Options

1. Step 1 needs `X` to be an lvalue, so the result is a prvalue (GCC,
   EDG, and every compiler in the one-class case).
2. Step 1 takes an xvalue that a `const T&` binds directly (Clang in
   the derived case).

## Provisional answer

Step 1 needs an lvalue (option 1). Two of three families agree, and
every compiler gives the one-class case this answer.

Kept open on 2026-11-05 under decision 0054. The review reads against the answer that the matrix keeps, so the question joins `pending.md` row 4 (item RV5).

## Opinions

### Gap review, 2026-11-05 (phase 12 text, expressions, overloading, and templates)

The source-reader gap review of the text phase 12 wrote on expressions, overloading, and templates (`reports/review/2026-11-05-gap-review-phase-12-expressions-overloading-templates.md`) reads the question: reads against the answer (RV5-B-1). The reference asks only that the other operand be an lvalue, and that the lvalue reference to its type bind directly to a glvalue. A reference to const binds directly to an xvalue of the same class or a derived class, so the xvalue operand converts to an lvalue, and both tests of the question get a const lvalue. Every compiler gives a prvalue in the one-class test, and Clang alone gives the lvalue in the derived-class test.
