# Q-0108: Must carries_dependency be on the first declaration of a function?

**Status:** provisional
**Affects:** ATTR-2, `tests/declarations/attributes/carries-dependency-redeclared.cpp`

## Context

A function is declared without `carries_dependency` and redeclared with it. Clang 18 and 22 and EDG 7.0 reject the redeclaration, as for `noreturn`. GCC 14 and 16 accept the redeclaration, because GCC ignores the attribute.

## Evidence

`tests/declarations/attributes/carries-dependency-redeclared.cpp`, with
an override for GCC.

## Options

1. Not valid, as for noreturn.

2. Valid; the attribute has no effect.

## Provisional answer

Yes, as for noreturn (option 1). Clang and EDG agree, and the rule matches noreturn.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page gives the answer. ATTR-P1 says that each standard attribute has a fixed set of places, and no principle says which declaration of a function must carry an attribute. GCC 16 is the only newest family that differs.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). ATTR-P1 failed step 3, because the model did not say which declaration of a function must carry an attribute. ATTR-P1 now says that `noreturn` and `carries_dependency` go on the first declaration of a function, and that a later declaration can repeat the attribute and cannot add it, because a call after the first declaration already relies on it. The new test `declarations/attributes/noreturn-repeated.cpp` checks the repeat. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses ATTR-P1.

Used on 2026-11-04 by the source-reader gap review of the text phase 8 wrote on the declarations and statements chapters (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`), under decision 0056. The review applied ATTR-P1 to the program of `carries-dependency-redeclared.cpp` without the rule text and reached the provisional answer: a later declaration cannot add the attribute, so the redeclaration is rejected.

Kept open on 2026-11-04 under decisions 0054 and 0055, because ATTR-P1 and ATTR-2 changed in this commit for two small points of the review on `fallthrough` and `nodiscard`. The question waits for the next use of the changed text.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. When any declaration of a function has `carries_dependency`, the first declaration must have it, and within one translation unit a diagnostic is required. A later change removes the attribute's requirements for the next standard, which matches GCC.

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on the declarations and statements chapters (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`) agrees with option 1.
