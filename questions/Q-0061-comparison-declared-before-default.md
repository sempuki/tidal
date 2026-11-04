# Q-0061: Can a friend comparison declared outside the class be defaulted in it?

**Status:** provisional
**Affects:** DCMP-3, `tests/classes/comparisons/declared-outside-then-defaulted.cpp`

## Context

A program declares a comparison operator at namespace scope. Then a class
declares the same function as a friend and defaults it. Clang and EDG
reject the default. GCC accepts it.

## Options

1. The first declaration of a defaulted comparison must be in its class.
2. An earlier declaration outside the class is valid.

## Provisional answer

No; the first declaration of a defaulted comparison must be in its class (option 1). Clang and EDG agree.

Kept open on 2026-11-03 under decision 0054, because no principle on the page states the case. DCMP-P1 describes what a defaulted comparison does and does not say where its first declaration must be. GCC 16 is the one newest family that differs.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) agrees with option 1. A class can default a comparison only where the class declares that function for the first time. A friend that defaults a function already declared at namespace scope breaks that rule.
