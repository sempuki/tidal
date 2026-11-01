# Q-0268: Are the constrained friend definitions of two class specializations two functions?

**Status:** provisional
**Affects:** CNS-32, `tests/templates/constraints/friend-definition-per-specialization.cpp`, `tests/templates/constraints/friend-distinct-from-namespace.cpp`

## Context

`Box<int>` and `Box<double>` each define `friend int touch(int)
requires (sizeof(T) > 1)`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts |
| GCC 14, 16, host | accepts |
| EDG 7.0 | rejects: touch has already been defined |

## Evidence

The first test above. In `friend-distinct-from-namespace.cpp`, a namespace template declares `touch` with the same parameters and a constraint that does not use `T`. Clang and GCC find a call ambiguous between the friend and that template. EDG accepts the call, so it treats the two as one template. INST-13 makes two definitions of an unconstrained friend not valid.

## Options

1. Two functions (Clang, GCC). The clause names `T`, so the two
   declarations have different constraints.
2. One function, defined twice (EDG).

## Provisional answer

Yes; Clang and GCC agree, EDG differs (option 1). Two of three implementations agree, and CNS-31 requires the friend to be a definition for this reason.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
