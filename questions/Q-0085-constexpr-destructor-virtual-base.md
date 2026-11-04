# Q-0085: Can a class with a virtual base have a constexpr destructor?

**Status:** settled
**Affects:** DER-12, `tests/classes/derived/constexpr-dtor-virtual-base.cpp`

## Context

A class with a virtual base declares a constexpr destructor. Clang and GCC reject the declaration. EDG 7.0 accepts it.

## Options

1. Not valid, as for constexpr constructors.
2. Valid.

## Answer

No, as for constexpr constructors (option 1). Clang and GCC agree, and option 1 matches the rule for constructors.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) supports this answer with no doubt, and DER-P4 gives the same answer: the position of a virtual base is decided at run time, so a class with a virtual base has no `constexpr` constructor or destructor. EDG 7.0 still differs. Tidal records EDG 7.0's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for EDG 7.0. The review notes a change after C++23 that lifts the limit. Q-0007 keeps such changes out of Tidal for now.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) agrees with option 1. A constexpr constructor or destructor that is not a template must belong to a class with no virtual base. As a side note, the review understands that a change for the next standard lifts the limit. Clang and GCC in the table still reject, so they do not follow that change.
