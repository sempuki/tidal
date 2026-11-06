# Q-0416: Does a use of a constant-initialized thread_local variable start the deferred construction of the other thread_local variables of its unit?

**Status:** provisional
**Affects:** OBJ-11, `tests/foundations/objects/thread-local-constant-use-no-construction.cpp`
**Source:** the gap review `reports/review/2026-11-05-gap-review-phase-10-classes-declarations-foundations.md`, section OBJ-11, from the probe that a clean-room writer ran for it.
**Waits for:** the maintainer (pending.md row 4)

## Context

A unit defines `thread_local int k = 5;`, which has constant initialization, and `thread_local C c;`, whose constructor records that it ran. `main` reads `k` and nothing else. OBJ-11 lets the implementation defer the construction of `c` to the first use of a `thread_local` variable of the unit that has dynamic initialization.

| Implementation | Result |
|---|---|
| Clang 22 | `c` is not constructed |
| GCC 16 | `c` is not constructed |
| EDG 7.0 | runs no program |

## Evidence

The test in **Affects**.

## Options

1. A use of a constant-initialized variable does not start the construction (Clang, GCC).
2. A use of any non-inline `thread_local` variable of the unit starts it (the gap review of 2026-11-05).

## Provisional answer

It does not start it (option 1). Every newest family that runs the program agrees. `pending.md` row 4 keeps the matrix's answer where at most one newest family follows the review.

## Opinions

### Gap review, 2026-11-05 (phase 10 text)

The source-reader gap review of the text phase 10 wrote (`reports/review/2026-11-05-gap-review-phase-10-classes-declarations-foundations.md`) reads the case as the question's option 2. See its section on OBJ-11.
