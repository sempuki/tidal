# Q-0060: Can a literal operator have a default argument?

**Status:** provisional
**Affects:** OPR-8, `tests/classes/operators/literal-default-arg.cpp`

## Context

A literal operator for strings takes a pointer and a length. A program
can declare the length parameter with a default argument. A literal never
uses the default, because the implementation always passes the length.
Clang and EDG reject the default argument. GCC accepts it.

## Options

1. A literal operator must not have a default argument.
2. A default argument is valid and has no effect on literals.

## Provisional answer

No; a literal operator must not have a default argument (option 1). Clang and EDG agree, and other operator functions have the
same limit.

Kept open on 2026-11-03 under decision 0054, because no principle on the page states the case. OPR-P2 bars default arguments for an operator function, and OPR-P3, which covers literal operators, does not mention default arguments. GCC 16 is the one newest family that differs.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) agrees with option 1. A default argument on any parameter of a literal operator makes the program not valid, and the C++23 text says so directly.
