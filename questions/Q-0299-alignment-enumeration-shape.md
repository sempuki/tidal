# Q-0299: Must an implementation reject a declaration of `std::align_val_t` with another shape?

**Status:** provisional
**Affects:** LSUP-5, `tests/foundations/language-support/alignment-enumeration-unscoped.cpp`, `tests/foundations/language-support/alignment-enumeration-wrong-underlying-type.cpp`, `tests/foundations/language-support/alignment-enumeration-class.cpp`

## Context

The implicit declarations of the aligned allocation functions use the
type `std::align_val_t`. A program that includes no header declares the
type itself. GCC 14 and 16 and EDG compare the program's declaration
with a declaration of their own. They reject an unscoped enumeration, a
scoped enumeration with another underlying type, and a class. Clang 18
and 22 accept each of these declarations. With a class, Clang still
passes the alignment to a program's aligned `operator new`.

## Evidence

Probes by the clean-room writer, kept as the tests above. Each test
declares one wrong shape. GCC says "scoped/unscoped mismatch",
"different underlying type", or "referred to as 'struct'". EDG says the
enumeration "was previously declared" another way.

## Options

1. LSUP-5 is a `reject` rule. Clang differs.
2. LSUP-5 is a `no-diagnostic` rule. A program with another shape is
   not valid, and an implementation need not say so.

## Provisional answer

Option 1. Two of the three implementations check the shape, and they
check it at the declaration, before any use.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
