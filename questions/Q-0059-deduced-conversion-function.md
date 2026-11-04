# Q-0059: Can a conversion function have a deduced return type?

**Status:** provisional
**Affects:** CVF-1, `tests/classes/conversion-functions/deduced-auto.cpp`

## Context

A conversion function can be written as `operator auto()`, with the
target type deduced from its `return` statements. Clang 18 and 22, EDG
7.0, and GCC 14 accept it. GCC 16 rejects it under `-pedantic-errors`
and accepts it otherwise.

## Options

1. A conversion function can have a deduced return type.
2. The target type must be written.

## Provisional answer

Yes; a conversion function can have a deduced return type (option 1). Most implementations accept `operator auto()`, and GCC 16
rejects it only as a pedantic diagnostic.

Kept open on 2026-11-03 under decision 0054, because no principle on the page states the case. CVF-P1 says that a conversion function declares no return type because its name gives the type, and does not say whether the name can be a placeholder. GCC 16 differs only under `-pedantic-errors`.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) agrees with option 1. Only a conversion function template is barred from a deduced return type, so a plain conversion function can deduce its type. GCC 16's pedantic error is a defect.
