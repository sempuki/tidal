# Q-0402: Must the ordering class be declared before a defaulted `<=>` is declared, or only before it is used?

**Status:** provisional
**Affects:** LSUP-2, `tests/foundations/language-support/ordering-declared-after-class.cpp`
**Source:** section 1.5 and the doubts of `reports/clean-room/roadmap-7/programmer.md`, from the reduction `ordering_declared_after_class.cpp`.
**Waits for:** a review (doubt)

## Context

LSUP-2 says that a program must first declare the ordering class that a
use of `<=>` yields. The rule did not say before what. In the test, a
class declares `auto operator<=>(const P&) const = default;`, the
program then declares `std::strong_ordering`, and `main` uses `<` on
two objects of the class.

| Implementation | Result |
|---|---|
| Clang 22 | rejects the defaulted declaration |
| GCC 16 | accepts and runs |
| EDG 7.0 | rejects the defaulted declaration |

## Evidence

The test in **Affects**, with an override for GCC.

## Options

1. The class is declared before the defaulted declaration, because the
   declaration deduces its return type there (Clang, EDG).
2. The class is declared before the first use of the defaulted `<=>`
   (GCC).

## Provisional answer

Before the defaulted declaration (option 1). Two of three families
agree, and the return type `auto` is deduced where the function is
declared as defaulted.

Kept open on 2026-11-05 under decision 0054, because the review reads the answer with a doubt (gap review of the text phase 9 wrote, item RV3).

Kept open on 2026-11-05 under decision 0054, because the review reads the answer with a doubt (gap review of the text phase 10 wrote, item RV4).

## Opinions

### Gap review, 2026-11-05 (declarations, statements, and foundations)

The source-reader gap review of the text phase 9 wrote on declarations, statements, and foundations (`reports/review/2026-11-05-gap-review-phase-9-declarations-statements-foundations.md`) reads the question: supports with a doubt. The reference makes lookups in the implicit definition of a defaulted comparison behave as from the function body, which sits inside the class, before the later declaration of the ordering class. So the ordering class must be declared before the defaulted declaration. The reference does not say plainly where the return type is deduced.

### Gap review, 2026-11-05 (phase 10 text, classes, declarations, foundations, overloading, and statements)

The source-reader gap review of the text phase 10 wrote on classes, declarations, foundations, overloading, and statements (`reports/review/2026-11-05-gap-review-phase-10-classes-declarations-foundations.md`) reads the question: supports with a doubt. The reference counts the deduced return type of a defaulted three-way comparison as an implicit use that the declaration must precede. It does not say where that use takes place: at the defaulted declaration, at the end of the class, or at the first use of the operator.
