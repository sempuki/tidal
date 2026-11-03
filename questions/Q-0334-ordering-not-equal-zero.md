# Q-0334: Does an ordering class with `!=` against a literal 0, and no `==`, serve a defaulted `<=>`?

**Status:** provisional
**Affects:** LSUP-12, DCMP-4, `tests/foundations/language-support/ordering-defaulted-not-equal-zero-only.cpp`

## Context

A defaulted `<=>` stops at the first part whose result is not equal
(DCMP-4). The implementation finds that part by comparing the result of
each part with a literal `0`. A program that includes no header
declares the ordering classes itself (LSUP-2), so the program decides
which comparisons with `0` exist.

The compilers test the result in two ways:

- GCC 14 and 16 apply `==` to the result and `0`. A class with only
  `!=` against `0` makes GCC delete the defaulted `<=>`.
- Clang 18 and 22 and EDG 7.0 apply `!=` to the result and `0`. A class
  with only `!=` against `0` serves them directly. A class with only
  `==` against `0` serves them through the rewritten candidate
  `!(v == 0)` (DCMP-6).

So a class with `==` against `0` serves every compiler, and a class
with only `!=` serves Clang and EDG.

## Evidence

- `tests/foundations/language-support/ordering-defaulted-not-equal-zero-only.cpp`:
  GCC rejects, with "no match for operator== (operand types are
  std::strong_ordering and int)". Clang and EDG accept.
- `tests/foundations/language-support/ordering-defaulted-needs-equal-zero.cpp`:
  with neither operator, every compiler rejects. EDG names `!=` in its
  message, and Clang names the operand types.
- `tests/foundations/language-support/ordering-complete-declaration.cpp`:
  with `==` against `0`, every compiler accepts.

## Options

1. The class must declare `==` against `0`. GCC conforms, and Clang and
   EDG accept a program that breaks LSUP-12. A program that follows the
   rule builds on every compiler.
2. The class must declare `==` or `!=` against `0`. Clang and EDG
   conform, and GCC rejects a valid program that declares only `!=`.

## Provisional answer

The class must declare `==` against `0` (option 1). The rule then
describes the one form that every compiler accepts, and the `==`
serves both ways of testing the result.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
