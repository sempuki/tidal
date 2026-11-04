# Q-0370: Can a definition outside the class add a default argument that makes a constructor a default, copy, or move constructor?

**Status:** provisional
**Affects:** DEFARG-12, DEFARG-2, `tests/declarations/functions/default-arguments/outside-default-makes-default-constructor.cpp`, `tests/declarations/functions/default-arguments/outside-default-makes-copy-constructor.cpp`, `tests/declarations/functions/default-arguments/outside-default-makes-move-constructor.cpp`

## Context

A definition of a member function outside its class can add a default
argument (DEFARG-2). For a constructor, the added default can change
the kind of the constructor:

- `S(int);` in the class and `S::S(int x = 5)` outside it make a
  default constructor.
- `S(const S&, int);` and `S::S(const S&, int = 0)` make a copy
  constructor.
- `S(S&&, int);` and `S::S(S&&, int = 0)` make a move constructor.

The class has already settled its special member functions at its
closing brace, so a later default argument would change them after the
fact.

| Compiler | Result for all three forms |
|---|---|
| Clang 18 and 22 | rejects ("addition of default argument on redeclaration makes this constructor a default constructor", and the same for copy and move) |
| EDG 7.0 | rejects ("added default arguments cannot result in declaring a default or copy constructor") |
| GCC 14 and 16 | accepts |

Every compiler accepts a default argument that a definition outside the
class adds to another member function.

## Evidence

The tests in Affects. The writer's probes in the Propose exercise X13
(probes `d8.cpp`, `d14.cpp`) and the writer's own probes give the same
results.

## Options

1. The program is not valid (Clang, EDG).
2. The program is valid, and the constructor gets its new kind from the
   definition on (GCC).

## Provisional answer

No; the program is not valid (option 1). Two of three families reject
it, and the kinds of a class's special member functions stay fixed after
the class definition.

Kept open on 2026-11-04 under decision 0054. The gap review of the rules that phase 7 rewrote supports the answer with no doubt, but DEFARG-2 changed in the same commit, to name the limit of the new rule DEFARG-13. The question settles after a reader or a review has used the changed rule (decision 0055). GCC 16 is the one newest family that differs.

## Opinions

### Gap review, 2026-11-04

The source-reader gap review of the rules that the phase 7 front-end and Propose repairs rewrote (`reports/review/2026-11-04-gap-review-phase-7-repairs.md`) agrees with option 1. The reference makes a program not valid when a default argument that a member definition outside the class adds makes a default, copy, or move constructor. It names the copy and move assignment operators too, which cannot have default arguments in any case.
