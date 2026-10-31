# Q-0117: Is a union with no active member a valid constant?

**Status:** provisional
**Affects:** CE-21, `tests/expressions/constant-expressions/result-union-no-active-member.cpp`

## Context

A union's `constexpr` default constructor initializes no member, so no
member is active. A `constexpr` variable of that union type has no
member with a value. CE-21 requires a value in each scalar part of a
constant, and in a union only the active member is a part.

Clang and EDG accept the variable. GCC 14 and 16 reject it and say
that the constructor leaves the variable incompletely initialized.
Every compiler accepts a constant union whose one active member has a
value.

## Evidence

`tests/RESULTS.md`, rows `result-union-no-active-member.cpp` and
`result-union-one-active-member.cpp`.

## Options

1. A union with no active member has no parts that need a value, so such
   a union is a valid constant. GCC differs.
2. A constant union must have an active member with a value. Clang and
   EDG differ.

## Provisional answer

Option 1. Two of three implementations accept the variable, and option 1
follows from CE-21 counting only the active member. The status is not
"compiler defect", because option 2 is a consistent reading too.

## Opinions
