# Q-0098: Do data members named begin and end select the member form of a range-based for?

**Status:** provisional
**Affects:** RFOR-2, `tests/statements/range-for/callable-data-members.cpp`, `tests/statements/range-for/data-member-begin-counts.cpp`

## Context

A class has members named begin and end. In one test both are data members of a callable class type. In the other, begin is an int data member and end is a member function. Clang and GCC use the member form whenever both names are members of any kind, so they accept the first test and reject the second. EDG 7.0 uses the member form only for member functions, so it does the opposite.

## Options

1. Any member named begin or end counts.

2. Only member functions count.

## Provisional answer

Yes; any member named begin or end counts (option 1). Clang and GCC agree.

## Opinions
