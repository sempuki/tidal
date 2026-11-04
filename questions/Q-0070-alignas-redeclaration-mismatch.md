# Q-0070: Must all declarations of a variable agree on alignas?

**Status:** settled
**Affects:** ALN-3, the four tests under Evidence

## Context

A variable is declared with one `alignas` and defined with a different one, or declared with `alignas` and defined without it. Clang 18 and 22 and EDG 7.0 reject both. GCC 14 and 16 accept both.

GCC also accepts two declarations of one class with different `alignas`,
and a class definition without the `alignas` of an earlier declaration.
Clang and EDG reject both.

## Evidence

`tests/declarations/alignment/redeclare-different.cpp`,
`definition-without-earlier-with.cpp`, `class-redeclare-different.cpp`,
and `class-definition-without-earlier-with.cpp`, each with an override
for GCC.

## Options

1. If a declaration has `alignas`, the definition must have the same alignment.
2. Declarations can differ.

## Answer

Yes; if a declaration has `alignas`, the definition must have the same alignment (option 1). Clang and EDG agree.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) supports this answer with no doubt, and ALN-P2 gives the same answer: alignment is a property of the entity, so every declaration of the entity agrees. GCC 16 still differs. Tidal records GCC 16's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for GCC 16.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. When any declaration of an entity states an alignment, every definition must state an equivalent one. A declaration that is not a definition may leave the specifier out. A diagnostic is required within one translation unit, and the rule covers variables and classes alike.
