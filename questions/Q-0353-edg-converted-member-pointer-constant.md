# Q-0353: Does EDG losing the member that a converted member pointer names during constant evaluation change MPTR-4?

**Status:** provisional
**Affects:** MPTR-3, MPTR-4, `tests/programs/W9/memptr.cpp`, `tests/programs/W9/mixin.cpp`

## Context

MPTR-4 converts a pointer to a member of a base `B` to a pointer to a
member of a derived class `D`. The converted pointer names the same
member. During constant evaluation EDG 7.0 loses the member in two
cases:

| Case | Clang 18, Clang 22, GCC 14, GCC 16 | EDG 7.0 |
|---|---|---|
| A pointer to a member function of `B`, read from a `constexpr` array, converted to a pointer to member of `D`, and called on a `D` | calls the function | the call does not change the object |
| A pointer to a data member of `B`, converted to a pointer to member of `D` by the `return` of a `constexpr` function, and used to assign to a `D` | assigns the member | the member keeps its value |

EDG gives the expected result when the pointer comes from a `constexpr`
variable, or when the conversion and the use are in one function.
Clang and GCC give the same results at run time.

## Evidence

`tests/programs/W9/memptr.cpp` and `tests/programs/W9/mixin.cpp`, the
W9 Write programs, with overrides for EDG. Probes by the clean-room
writer on the matrix containers gave the table above.

## Options

1. A converted member pointer names the same member wherever its value
   goes (MPTR-4), and constant evaluation follows run time (CE-P1).
   EDG's result is a defect.
2. A converted member pointer that passes through an object or a
   function result has no reliable value during constant evaluation.

## Provisional answer

No; the converted pointer names the member, and EDG differs (option 1).
MPTR-4 states no condition on where the pointer's value comes from, and
Clang and GCC agree at run time and in constant evaluation.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
