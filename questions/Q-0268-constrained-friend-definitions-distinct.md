# Q-0268: Are the constrained friend definitions of two class specializations two functions?

**Status:** provisional
**Affects:** CNS-32, `tests/templates/constraints/friend-definition-per-specialization.cpp`

## Context

`Box<int>` and `Box<double>` each define `friend int touch(int)
requires (sizeof(T) > 1)`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts |
| GCC 14, 16, host | accepts |
| EDG 7.0 | rejects: touch has already been defined |

## Evidence

The test above. INST-13 makes two definitions of an unconstrained friend not valid.

## Options

1. Two functions (Clang, GCC). The clause names `T`, so the two
   declarations have different constraints.
2. One function, defined twice (EDG).

## Provisional answer

Option 1. Two of three implementations agree, and CNS-31 requires the friend to be a definition for this reason.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
