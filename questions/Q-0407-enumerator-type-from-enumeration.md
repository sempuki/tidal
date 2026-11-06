# Q-0407: Inside the enumerator list, does an enumerator whose initializer has an unscoped enumeration type have that enumeration type, or its underlying type?

**Status:** provisional
**Affects:** ENUM-10, ENUM-P1, `tests/declarations/enumerations/enumerator-type-enum-initializer.cpp`, `tests/declarations/enumerations/enumerator-type-first-without-initializer.cpp`
**Source:** the gap review `reports/review/2026-11-05-gap-review-phase-9-declarations-statements-foundations.md`, on ENUM-P1, ENUM-10, from the probes that a clean-room writer ran for it.
**Waits for:** the maintainer (pending.md row 4)

## Context

An enumeration without a fixed underlying type has an enumerator whose initializer is an enumerator of another unscoped enumeration `E1` with underlying type `short`. Inside the list, Clang 22, GCC 16, and EDG 7.0 give the enumerator the type `E1`. The gap review reads that it has the underlying type of `E1`. The review also reads that the first enumerator without an initializer has a signed integer type that the implementation chooses; every baseline compiler gives `int`, as ENUM-10 says.

## Evidence

The tests in **Affects**. Every baseline compiler agrees with the answer.

## Options

1. The enumeration type of the initializer, and `int` for a first enumerator without an initializer (every baseline compiler).
2. The underlying type of that enumeration, and a signed type that the implementation chooses (the gap review).

## Provisional answer

The enumeration type, and `int` (option 1). Every newest family gives this answer. `pending.md` row 4 keeps the matrix's answer where at most one
newest family follows the review.

## Opinions

### Gap review, 2026-11-05

The source-reader gap review (`reports/review/2026-11-05-gap-review-phase-9-declarations-statements-foundations.md`) reads the reference as option 2. See its section on ENUM-P1, ENUM-10.
