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

Kept open on 2026-11-03 under decision 0054, because LSUP-5 changed on the same day, and a question settles only after a reader or a review has used the changed rule (decision 0055). Every compiler's result fits the rule.

## Opinions

### Gap review, 2026-11-04

The source-reader gap review of the rules that phase 7 added (`reports/review/2026-11-04-gap-review-phase-7-additions.md`) reads LSUP-5 against the classification. In the reference, a program that adds a declaration to namespace `std` has undefined behavior unless a rule allows it, and no rule allows a declaration of `std::align_val_t`. A note says that the implicit declarations of the allocation functions do not declare the type. So every shape that a program declares is undefined, the right one included. The `no-diagnostic` class matches the compilers, and LSUP-P2's model of a program that supplies a library declaration is Tidal's own choice.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-11-03

The source-reader gap review of the object model (`reports/review/2026-11-03-gap-review-object-model.md`) reads the reference as supporting option 2. In the reference, a program that adds a declaration to namespace `std` has undefined behavior, so no shape needs a diagnostic. The rules of the language-support page have no counterpart in the reference, and decision 0041 makes them Tidal choices.
