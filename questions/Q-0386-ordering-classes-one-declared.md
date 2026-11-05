# Q-0386: Must a program declare all three ordering classes before a use of `<=>` that yields only one of them?

**Status:** provisional
**Affects:** LSUP-2, `tests/foundations/language-support/ordering-strong-only-builtin.cpp`, `tests/foundations/language-support/ordering-strong-only-builtin-runtime.cpp`, `tests/foundations/language-support/ordering-strong-only-defaulted.cpp`, `tests/overloading/overload-resolution/builtin-three-way.cpp`, `tests/overloading/overload-resolution/builtin-three-way-mixed.cpp`
**Source:** section 1.4, item 7, and section 6 of `reports/clean-room/roadmap-6/programmer.md`, from the probes that a clean-room writer ran for them.

## Context

LSUP-2 says that a program that uses `<=>` on built-in operands, or a
defaulted `<=>`, declares `std::strong_ordering`,
`std::weak_ordering`, and `std::partial_ordering` first. Each test
below declares only `std::strong_ordering`, and each use of `<=>`
yields that class.

| Use of `<=>` | Clang 22, GCC 16 | EDG 7.0 |
|---|---|---|
| a built-in `<=>` on two `int` values, evaluated as a constant | accept | reject |
| a built-in `<=>` on two `int` parameters, evaluated at run time | accept | accept |
| a defaulted `<=>` whose members are `int`, called at run time | accept | reject |

EDG's message names each class that the program does not declare. Clang
18 and GCC 14 accept the three programs, as Clang 22 and GCC 16 do.

`builtin-three-way.cpp` and `builtin-three-way-mixed.cpp` test the
result types of the built-in candidates (OVL-9) with `<=>` inside
`decltype`. Both declared two of the three classes, and every compiler
accepted them. Both now declare all three classes, so they meet LSUP-2
and still test OVL-9.

## Evidence

The three LSUP-2 tests in **Affects**. The first and the third carry
overrides for Clang and GCC. The run-time test carries overrides for
all three families. Before the change, the two OVL-9 tests declared
`std::strong_ordering` and `std::partial_ordering` only, and EDG
accepted them.

## Options

1. Every use of `<=>` on built-in operands and every defaulted `<=>`
   needs all three classes (the rule as written). No compiler checks
   the run-time form.
2. Only the uses that EDG checks need all three classes: a built-in
   `<=>` evaluated as a constant and a defaulted `<=>`. Other uses need
   only the class that they yield.
3. Each use needs only the class that it yields (Clang, GCC).

## Provisional answer

Yes; every such use needs all three classes (option 1). A program that
declares all three classes works on every compiler for every use. A
rule that depends on whether a comparison is evaluated as a constant is
harder for a programmer to follow, and a later change of either
program shape would move the program across the line.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
