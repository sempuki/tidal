# Q-0401: Does a reference bind directly to a class prvalue in steps 1 and 2 of COND-16?

**Status:** provisional
**Affects:** COND-16, `tests/expressions/conditional/class-derived-prvalue-against-base-xvalue.cpp`
**Source:** finding G148 of `reports/clean-room/rust-frontend/report-21.md`, from the probe that a clean-room writer ran for it.
**Waits for:** a first review

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

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
