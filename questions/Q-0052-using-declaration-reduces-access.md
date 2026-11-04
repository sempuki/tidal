# Q-0052: Can a using-declaration make a public base member private?

**Status:** settled
**Affects:** ACC-9, `tests/classes/access/using-reduces-access.cpp`

## Context

A derived class with a public base can redeclare a base member with a using-declaration under a `private` label. Clang and GCC then reject a use of that member through the derived class from outside. EDG 7.0 accepts the use. This result suggests that EDG keeps the access that the member has through the base.

## Options

1. The using-declaration sets the access, so the use is rejected.
2. The more permissive of the two accesses applies, so the use is accepted.

## Answer

Yes; the using-declaration sets the access, so the use is rejected (option 1). Clang and GCC agree, and the same rule increases access in `using-changes-access.cpp` on all compilers.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) supports this answer with no doubt, and ACC-P4 gives the same answer: a using-declaration can raise or lower the access of a base member that the derived class can reach. EDG 7.0 still differs. Tidal records EDG 7.0's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for EDG 7.0.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) agrees with option 1. Access is checked on the using-declaration that lookup finds, with the access of the place where the derived class declares it. A private using-declaration makes a use from outside the class not valid.
