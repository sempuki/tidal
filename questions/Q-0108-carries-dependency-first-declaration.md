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

Kept open on 2026-11-03 under decision 0054, because no principle on the page gives the answer. ATTR-P1 says that each standard attribute has a fixed set of places, and no principle says which declaration of a function must carry an attribute. GCC 16 is the only newest family that differs.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. When any declaration of a function has `carries_dependency`, the first declaration must have it, and within one translation unit a diagnostic is required. A later change removes the attribute's requirements for the next standard, which matches GCC.
