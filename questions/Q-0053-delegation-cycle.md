# Q-0053: Must an implementation diagnose an indirect delegation cycle?

**Status:** provisional
**Affects:** CTOR-14, `tests/classes/constructors/delegating-cycle.cpp`

## Context

Two constructors of a class can delegate to each other. Such a program
can never finish construction. Clang and EDG reject it. GCC accepts it
without a diagnostic. All three reject a constructor that delegates
directly to itself.

## Options

1. The program is not valid, and an implementation need not diagnose an
   indirect cycle (`no-diagnostic`).
2. Implementations must reject every cycle (`reject`).

## Provisional answer

Option 1. GCC accepts it, so a `reject` rule would make a current
implementation non-conforming. A cycle through constructors in other
translation units cannot be found during translation.

## Opinions
