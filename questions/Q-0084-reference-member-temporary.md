# Q-0084: Must an implementation reject a constructor that binds a reference member to a temporary?

**Status:** provisional
**Affects:** CTOR-17, `tests/classes/constructors/reference-member-binds-temporary.cpp`, `tests/classes/constructors/reference-default-member-binds-temporary.cpp`

## Context

A constructor binds a reference member to a temporary, through a
member initializer or a default member initializer. The temporary ends with
the constructor, so the reference dangles. Clang rejects both forms. GCC
accepts with a warning. EDG accepts.

## Options

1. The program is not valid, and an implementation need not diagnose it
   (no-diagnostic).
2. Implementations must reject the program.
3. The program is valid, and a later use of the reference is undefined.

## Provisional answer

Option 1. Clang diagnoses the program and GCC warns. All three
still conform.

## Opinions
