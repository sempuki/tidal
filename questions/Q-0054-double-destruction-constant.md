# Q-0054: Is a second destruction rejected during constant evaluation?

**Status:** settled
**Affects:** DTOR-7, `tests/classes/constructors/double-destroy-constant.cpp`

## Context

A function calls the destructor of an automatic object explicitly. The
object is then destroyed again at the end of its block. During constant
evaluation, Clang and GCC reject the second destruction. EDG 7.0 accepts
it. The split is the same for a class with a trivial destructor
(`tests/classes/constructors/double-destroy-trivial-constant.cpp`).

The same split holds when the called function destroys its parameter
and the caller then destroys the parameter again
(`tests/foundations/values/parameter-destroyed-by-callee-constant.cpp`,
VC-10). Clang and GCC reject the second destruction during constant
evaluation, and EDG 7.0 accepts it.

## Options

1. The behavior is undefined, so constant evaluation rejects it.
2. Constant evaluation accepts it.

## Answer

Yes; the behavior is undefined, so constant evaluation rejects it (option 1). Clang and GCC agree. A use of the object after the first
destruction is rejected by all compilers.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) supports this answer with no doubt, and CTOR-P4 gives the same answer: after the destructor runs, the object is gone, and destroying the object again is undefined, so constant evaluation rejects the second destruction. EDG 7.0 still differs. Tidal records EDG 7.0's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for EDG 7.0.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) agrees with option 1. A second destructor call on an object whose lifetime has ended is undefined, and an explicit call followed by the implicit one at the end of the block is a named case of it. Undefined behavior of the core language keeps an expression from being constant.
