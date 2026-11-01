# Q-0041: May a jump skip a class object that has a non-trivial destructor and no initializer?

**Status:** provisional
**Affects:** JMP-6, SW-5, `tests/statements/jumps/goto-past-nontrivial-destructor.cpp`, `tests/statements/jumps/goto-skips-nontrivial-destructor.cpp`, `tests/statements/switch/case-skips-nontrivial-destructor.cpp`

## Context

A `goto` or a `case` label skips the declaration, without an
initializer, of a variable whose class has a trivial default
constructor and a user-provided destructor, such as `Tidy t;`.

| Jump | Clang 22, 18 | GCC 16, 14 | EDG 7.0 |
|---|---|---|---|
| `goto` past the declaration | rejects | accepts | rejects |
| `case` label past the declaration | rejects | accepts | rejects |

A jump past a class with a trivial constructor and a trivial destructor
is accepted everywhere.

SW-5 refers to JMP-6 for the types that a `case` label can skip. Q-0097
asked the same question for both rules and is a duplicate of this
question.

## Evidence

The three tests in **Affects**. Each has an override for GCC.

## Options

1. Rejected: only types with a trivial default constructor and a
   trivial destructor can be skipped. GCC's acceptance is a possible
   defect.
2. Accepted when the constructor is trivial.

## Provisional answer

No; JMP-6 and SW-5 reject the jump; GCC accepts it (option 1). Two of three implementations reject the jump, JMP-6 lists
only types whose destructor is trivial, and the review summary
describes the same limit.

## Opinions
