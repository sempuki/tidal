# Q-0418: In step 1 of COND-16, can an xvalue operand bind the lvalue reference to the other operand's type?

**Status:** provisional
**Affects:** COND-16, `tests/expressions/conditional/class-derived-xvalue-const-base-lvalue.cpp`, `tests/expressions/conditional/class-const-lvalue-against-xvalue.cpp`
**Source:** finding I4-2 of `reports/clean-room/I4/report.md`, from the probes that a clean-room writer ran for it.
**Waits for:** a first review

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

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
