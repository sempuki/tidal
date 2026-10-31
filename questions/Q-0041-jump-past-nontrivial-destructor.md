# Q-0041: May a jump skip a class object that has a non-trivial destructor and no initializer?

**Status:** provisional
**Affects:** JMP-6, `tests/statements/jumps/goto-past-nontrivial-destructor.cpp`

## Context

A `goto` that skips `Tidy t;`, where `Tidy` has a trivial default constructor but a user-provided destructor, is rejected by Clang 22 and 18 and EDG 7.0. GCC 16 and 14 accept it. A jump past a class with a trivial constructor and trivial destructor is accepted everywhere.

## Options

1. Rejected: only types with a trivial default constructor and a trivial destructor can be skipped. GCC's acceptance is a possible defect.
2. Accepted when the constructor is trivial.

## Provisional answer

Option 1. Two of three implementations reject the jump, and the review summary describes the same limit.

## Opinions
