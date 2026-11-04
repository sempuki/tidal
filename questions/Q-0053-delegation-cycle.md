# Q-0053: Must an implementation diagnose an indirect delegation cycle?

**Status:** settled
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

## Answer

No; the program is not valid, and an indirect cycle is `no-diagnostic` (option 1). GCC accepts such a program, so a `reject` rule would make a
current implementation non-conforming. A cycle through constructors in
other translation units cannot be found during translation.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) supports this answer with no doubt, and CTOR-P6 gives the same answer: a constructor cannot delegate back to itself, directly or through a cycle, and an implementation need not find a cycle that runs through several constructors. No newest family differs from the classification, because CTOR-14 needs no diagnostic. GCC 16 accepts the cycle, which the rule allows, and the test keeps its override for GCC.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) agrees with option 1. A constructor that delegates to itself, directly or through other constructors, makes the program not valid, and no diagnostic is required.
