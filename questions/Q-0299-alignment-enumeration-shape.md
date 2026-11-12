# Q-0299: Must an implementation reject a declaration of `std::align_val_t` with another shape?

**Status:** provisional
**Affects:** LSUP-5, `tests/foundations/language-support/alignment-enumeration-unscoped.cpp`, `tests/foundations/language-support/alignment-enumeration-wrong-underlying-type.cpp`, `tests/foundations/language-support/alignment-enumeration-class.cpp`
**Waits for:** a review (doubt)

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

Used on 2026-11-05 in batch PU3 (`reports/clean-room/PU3/report.md`). LSUP-P2 decided none of the three programs, because it gives no kind or underlying type for `std::align_val_t` (finding PU3-4).

Kept open on 2026-11-05 under decision 0054, because the review reads the answer with a doubt (gap review of the text phase 10 wrote, item RV4).

Kept open on 2026-11-11 under decision 0054 (decision 0066, item DQ). The second review supports the answer with a narrower doubt (DQ-15). Clang 22 alone accepts, and the no-diagnostic rule lets either result stand. The doubt is only the declaration of the right shape, which the reference also leaves undefined and which LSUP-5 accepts under decision 0041.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the object model (`reports/review/2026-11-03-gap-review-object-model.md`) reads the reference as supporting option 2. In the reference, a program that adds a declaration to namespace `std` has undefined behavior, so no shape needs a diagnostic. The rules of the language-support page have no counterpart in the reference, and decision 0041 makes them Tidal choices.

### Gap review, 2026-11-04

The source-reader gap review of the rules that phase 7 added (`reports/review/2026-11-04-gap-review-phase-7-additions.md`) reads LSUP-5 against the classification. In the reference, a program that adds a declaration to namespace `std` has undefined behavior unless a rule allows it, and no rule allows a declaration of `std::align_val_t`. A note says that the implicit declarations of the allocation functions do not declare the type. So every shape that a program declares is undefined, the right one included. The `no-diagnostic` class matches the compilers, and LSUP-P2's model of a program that supplies a library declaration is Tidal's own choice.

### Gap review, 2026-11-05 (declarations, statements, and foundations)

The source-reader gap review of the text phase 9 wrote on declarations, statements, and foundations (`reports/review/2026-11-05-gap-review-phase-9-declarations-statements-foundations.md`) used LSUP-P2 on `foundations/language-support/alignment-enumeration-unscoped.cpp`, `alignment-enumeration-wrong-underlying-type.cpp`, and `alignment-enumeration-class.cpp`, without the rule text. The model's answer: The program's declaration disagrees with the library on the kind of the name, so the program is not valid. The principle does not say whether a diagnostic is needed. Matches the provisional answer: partly (it gives "not valid" and leaves the diagnostic open). Reference verdict: supports with a doubt.

### Gap review, 2026-11-05 (phase 10 text, classes, declarations, foundations, overloading, and statements)

The source-reader gap review of the text phase 10 wrote on classes, declarations, foundations, overloading, and statements (`reports/review/2026-11-05-gap-review-phase-10-classes-declarations-foundations.md`) used LSUP-P2 on `alignment-enumeration-unscoped.cpp`, without the rule text. The model's answer: The type is declared unscoped, which is another kind. The program is not valid and needs no diagnostic. An implementation can reject the declaration where it stands, so the GCC and EDG rejection and the Clang acceptance both conform. Matches the provisional answer: yes. Reference verdict: supports with a doubt. The review also used LSUP-P2 on `alignment-enumeration-wrong-underlying-type.cpp`, without the rule text. The model's answer: The type is scoped but has a one-byte underlying type, which is another underlying type. The program is not valid and needs no diagnostic, and rejection at the declaration is allowed. Matches the provisional answer: yes. Reference verdict: supports with a doubt. The review also used LSUP-P2 on `alignment-enumeration-class.cpp`, without the rule text. The model's answer: The type is declared as a class, which is another kind. The program is not valid and needs no diagnostic, and rejection at the declaration is allowed. Matches the provisional answer: yes. Reference verdict: supports with a doubt.

### Second review of the doubt-held questions, 2026-11-11

The source-reader second review of the questions held only by a doubt (`reports/review/2026-11-11-second-review-doubt-held-questions.md`) reads the question: supports with a doubt that remains (DQ-15). A declaration that a program adds to namespace `std` is undefined unless a rule allows it, and no rule allows this one. The implicit declarations of the allocation functions do not declare the type. So no shape needs a diagnostic, and the answer holds. The doubt is now only the class of the right shape, which the reference makes undefined too. LSUP-5 and LSUP-P2 treat that declaration as valid, which is Tidal's choice under decision 0041.
