# Q-0382: Is `override` after the name of a class an identifier?

**Status:** provisional
**Affects:** KW-3, `tests/foundations/keywords/override-after-class-name.cpp`, `tests/foundations/keywords/override-after-complete-class-variable.cpp`
**Source:** finding X18-3 of `reports/clean-room/X18/report.md`, from the matrix run of the tests that a clean-room writer added for it.

## Context

KW-3 gives `override` a special meaning only after the declarator of a
member function. After the name of a class, the word is an identifier.
So `struct D override { 3 };`, for a complete class `D`, declares a
variable named `override` with a braced initializer.

| Compiler | `struct D override { 3 };` |
|---|---|
| Clang 18, Clang 22 | a variable |
| GCC 16 | a variable |
| EDG 7.0 | a variable |
| GCC 14 | rejected: `override` cannot mark a class |

## Evidence

The tests in **Affects**, with an override for GCC 14.

## Options

1. `override` after a class name is an identifier (Clang, GCC 16, EDG).
2. `override` after a class name begins a class head and is not valid
   there (GCC 14).

## Provisional answer

An identifier (option 1). Every newest compiler agrees, and only `final`
marks a class (DER-1).

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
