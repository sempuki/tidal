# Q-0401: Does a reference bind directly to a class prvalue in steps 1 and 2 of COND-16?

**Status:** provisional
**Affects:** COND-16, `tests/expressions/conditional/class-derived-prvalue-against-base-xvalue.cpp`
**Source:** finding G148 of `reports/clean-room/rust-frontend/report-21.md`, from the probe that a clean-room writer ran for it.
**Waits for:** the maintainer (pending.md row 4)

## Context

Step 2 of COND-16 asks whether a `T&&` binds directly to the operand
`X` when the other operand `Y` is an xvalue. The glossary says that a
reference binds directly when no temporary object is in between. A
class prvalue becomes a temporary object when a reference binds to it.
So the step can be read both ways.

The test gives a conditional expression a `Derived` prvalue and a
`Base` xvalue.

| Implementation | Result |
|---|---|
| Clang 22 | a prvalue of `Base` |
| GCC 16 | an xvalue of `Base` |
| EDG 7.0 | a prvalue of `Base` |

## Evidence

The test in **Affects**, with an override for GCC.

## Options

1. A reference does not bind directly to a class prvalue in steps 1
   and 2. The `Derived` operand converts by step 3, and the result is a
   prvalue of `Base` (Clang, EDG).
2. A `T&&` binds directly to a class prvalue of `T` or of a derived
   class. The result is an xvalue of `Base` (GCC).

## Provisional answer

No; the operand converts by step 3, and the result is a prvalue
(option 1). Two of three families agree, and the glossary's definition
of binding directly leaves out a temporary object in between.

Kept open on 2026-11-05 under decision 0054, because the review reads against the answer (gap review of the text phase 9 wrote, item RV3). GCC 16 follows the review, and Clang 22 and EDG 7.0 follow the answer, so the question waits on the maintainer (`documentation/decisions/pending.md`, row 4).

## Opinions

### Gap review, 2026-11-05 (expressions and templates)

The source-reader gap review of the text phase 9 wrote on expressions and templates (`reports/review/2026-11-05-gap-review-phase-9-expressions-templates.md`) reads the question: reads against. The xvalue step asks only that the reference bind directly. Binding an rvalue reference to a class prvalue of a derived class counts as direct, so the result is an xvalue of the base.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
