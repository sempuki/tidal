# Q-0382: Is `override` after the name of a class an identifier?

**Status:** settled
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

## Answer

An identifier (option 1). Every newest compiler agrees, and only `final`
marks a class (DER-1).

Settled on 2026-11-04 under decision 0051, because the newest version of each baseline family (Clang 22, GCC 16, EDG 7.0) agrees with this answer, and the two gap reviews that read the question support it with no doubt. GCC 14 still rejects the variable, and the tests keep their overrides for GCC 14.

## Opinions

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on classes and overloading (`reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`) agrees with option 1. A class head takes only the `final` marker after the class name, and a word with a special meaning is read as a plain identifier wherever the special meaning does not fit. So the declaration names a variable of the complete class with a braced initializer, and GCC 14 is wrong.

### Gap review, 2026-11-04 (expressions, foundations, and templates)

The source-reader gap review of the text phase 8 wrote on expressions, foundations, exceptions, and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`) agrees with option 1. The only class marker in the grammar is `final`, so `override` after a class name is an identifier.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
