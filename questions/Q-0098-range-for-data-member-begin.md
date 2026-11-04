# Q-0098: Do data members named begin and end select the member form of a range-based for?

**Status:** settled
**Affects:** RFOR-2, `tests/statements/range-for/callable-data-members.cpp`, `tests/statements/range-for/data-member-begin-counts.cpp`

## Context

A class has members named begin and end. In one test both are data members of a callable class type. In the other, begin is an int data member and end is a member function. Clang and GCC use the member form whenever both names are members of any kind, so they accept the first test and reject the second. EDG 7.0 uses the member form only for member functions, so it does the opposite.

## Options

1. Any member named begin or end counts.

2. Only member functions count.

## Answer

Yes; any member named begin or end counts (option 1). Clang and GCC agree.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) supports this answer with no doubt, and RFOR-P1 gives the same answer: the loop finds `begin` and `end` as members if the class has both names as members of any kind. EDG 7.0 still differs. Tidal records EDG 7.0's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for EDG 7.0.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) agrees with option 1. The member form applies when a search in the class finds at least one declaration of each name, whatever kind of member it is.
