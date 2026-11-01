# Q-0181: Does a class prvalue initialize a by-value explicit object parameter without a copy or move?

**Status:** provisional
**Affects:** CLS-13, `tests/classes/members/explicit-object-from-prvalue.cpp`

## Context

A member function takes its object by value, as `this Fixed self`. The
call is `Fixed{}.get()`, and `Fixed` has deleted copy and move
constructors. Clang 22, the host Clang, GCC 14, GCC 16, the host GCC, and
EDG accept the call, and the run destroys one object. Clang 18 rejects
the call ("call to deleted constructor").

## Evidence

The test above, on the eight baseline compilers.

## Options

1. The prvalue initializes the parameter directly, as any prvalue of the
   same type does (VC-4).
2. The call makes a temporary and copies or moves it into the parameter.

## Provisional answer

Yes; Clang 18's rejection is fixed in Clang 22 (option 1). Every other implementation agrees, and Clang 22 accepts the
call, so the Clang 18 rejection is a defect that a later version fixes.

Reopened on 2026-11-01. The source-reader gap review of question cluster 1 (`reports/review/2026-11-01-gap-review-question-cluster-1.md`) reads the C++23 text as giving option 2, a copy or move from a materialized temporary. It reports that the newest compilers follow a later change to the language, which gives option 1. The specification follows the matrix until the maintainer decides whether Tidal follows that later change (Q-0007).

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
