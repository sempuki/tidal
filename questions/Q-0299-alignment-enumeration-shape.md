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

No; LSUP-5 needs no diagnostic, GCC and EDG reject another shape, and Clang accepts it (option 2). A program with another shape is not valid. The tests expect the rejection that GCC and EDG give, and record Clang's acceptance with an override.

Changed on 2026-11-03. The source-reader gap review of the object model (`reports/review/2026-11-03-gap-review-object-model.md`) reads the reference as supporting option 2. The reading matches Clang 22, which accepts each shape, and every result of the matrix fits a rule that needs no diagnostic. LSUP-P2 asks the program and the library to agree on each name, which is a requirement on the program. LSUP-5 is now a `no-diagnostic` rule in the Implementation Requirements segment.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-11-03

The source-reader gap review of the object model (`reports/review/2026-11-03-gap-review-object-model.md`) reads the reference as supporting option 2. In the reference, a program that adds a declaration to namespace `std` has undefined behavior, so no shape needs a diagnostic. The rules of the language-support page have no counterpart in the reference, and decision 0041 makes them Tidal choices.
