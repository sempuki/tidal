# Q-0377: Does the body of a class defined with a qualified name see the members of the qualifier without qualification?

**Status:** settled
**Affects:** NAM-21, LSUP-3, `tests/foundations/names/definition-class-body-sees-namespace.cpp`, `tests/foundations/language-support/tuple-protocol-page-example.cpp`
**Source:** reports/clean-room/roadmap-5/programmer.md, section 1.4 (the `prelude.cpp` row) and section 1.5

## Context

NAM-21 lists the parts of a declaration that come after a qualified
name and see the members of the qualifier: the parameters, an array
bound, an exception specification, a trailing return type, the
function body, and the initializer. The list does not name the body of
a class that the declaration defines.

A class can be defined with a qualified name in two common forms. A
class declared in a namespace is defined outside the namespace, as in
`struct N::C { ... };`. An explicit or partial specialization of a
library template is defined outside namespace `std`, as in the tuple
protocol example of the language-support page, whose specialization
`std::tuple_size<Range>` uses `size_t` without qualification.

Every compiler in the matrix accepts both forms. The clean-room front
end rejects the tuple protocol example: it looks up `size_t` in the
class body and does not search `std`.

## Evidence

`tests/foundations/names/definition-class-body-sees-namespace.cpp` and
`tests/foundations/language-support/tuple-protocol-page-example.cpp`
run to result 0 on the eight baseline compilers and the two report
compilers.

## Options

1. The class body comes after the qualified name and sees the members
   of the qualifier, as the function body does. Every compiler behaves
   this way.
2. NAM-21 covers only functions and variables. A class body needs
   qualification. No compiler behaves this way, and the page example
   would be invalid.

## Answer

Yes; the class body sees the members of the qualifier (option 1).
Every compiler agrees, and the language-support page's own example
depends on the answer. NAM-21 now names the class body.

Kept open on 2026-11-04 under decision 0054, because no review has read the question. Item RV2 of decision 0056 reads it.

Kept open on 2026-11-04 under decision 0055, because NAM-21 changed in this commit. The gap reviews of the text phase 8 wrote on declarations and statements and on expressions, foundations, and templates both support option 1. NAM-21 now also names the base clause of the class and the enumerator list of an enumeration defined with a qualified name. The question waits for a reader or a reviewer to use the changed rule. No newest family differs.

Used on 2026-11-04 by the principle-use batch PU1 (`reports/clean-room/PU1/`), under decision 0056. A clean-room reader applied the principles, without the rule text, to the programs of `definition-class-body-sees-namespace.cpp` and `tuple-protocol-page-example.cpp`. Each answer gives the outcome that the tests expect (the program runs). The answer for `definition-class-body-sees-namespace.cpp` names NAM-P5. The answer for `tuple-protocol-page-example.cpp` names SB-P2 and LSUP-P1 and not NAM-P5. The use does not support the answer.

Kept open on 2026-11-04 under decisions 0055 and 0056, because the use in batch PU1 does not support the answer: one of the two answers does not name NAM-P5. The question also waits for a use of the changed NAM-21. No newest family differs.

Kept open on 2026-11-04 under decision 0055, because LSUP-P1, which the answer for `tuple-protocol-page-example.cpp` named, changed in this commit after finding PU1-3. The question waits for the next use of NAM-P5 or LSUP-P1, and for a use of the changed NAM-21. No newest family differs.

Used on 2026-11-04 by the principle-use batch PU2 (`reports/clean-room/PU2/`), under decision 0056. A clean-room reader applied LSUP-P1 and NAM-P5, without the rule text, to the programs of `definition-class-body-sees-namespace.cpp` and `tuple-protocol-page-example.cpp`. The batch key lists both principles. The answer for `definition-class-body-sees-namespace.cpp` names NAM-P5, and the answer for `tuple-protocol-page-example.cpp` names LSUP-P1. Each answer gives the outcome that the tests expect (the program runs). The use supports the answer.

Settled on 2026-11-04 under decision 0054. The use in batch PU2 supports the answer, the two gap reviews of 2026-11-04 that read the question support it with no doubt, and NAM-P5 gives the same answer by the test of the style guide (section 3). The answer for `definition-class-body-sees-namespace.cpp` applies NAM-P5 to the body of a class defined with a qualified name, the case that the changed NAM-21 adds. No newest family differs.

## Opinions

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on the declarations and statements chapters (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`) agrees with option 1. A declaration that redeclares or specializes a member of a namespace or class with a qualified name puts the part after that name into the scope of the qualifier. The rule names the head of a class and of an enumeration as well as a declarator, and leaves out friend declarations. So the body of a class defined with a qualified name sees the members of the qualifier, and so does a specialization of a library template defined outside namespace `std`. The same rule covers the base clause of such a class, and the base and enumerator list of such an enumeration, which NAM-21 did not name.

### Gap review of expressions, foundations, and templates, 2026-11-04

The source-reader gap review of the text phase 8 wrote on expressions, foundations, and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`) agrees with option 1, for the same reason, and notes that a specialization is covered, so the tuple protocol example of the language-support page is valid. It adds that the body of an enumeration defined with a qualified name is treated the same way.

### Gap review of classes and overloading, 2026-11-04

The source-reader gap review of the text phase 8 wrote on the classes and overloading chapters (`reports/review/2026-11-04-gap-review-phase-8-classes-overloading.md`) names the question only to say that it affects no rule on its pages, and gives no reading.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
