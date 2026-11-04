# Q-0056: Is delete through a base pointer without a virtual destructor rejected during constant evaluation?

**Status:** settled
**Affects:** VIRT-12, `tests/classes/virtual/nonvirtual-dtor-delete-constant.cpp`

## Context

A program creates a derived object with `new` and deletes it through a
pointer to a base whose destructor is not virtual. During constant
evaluation, Clang and EDG reject the deletion. GCC 14 and 16 accept it.

## Options

1. The behavior is undefined, so constant evaluation rejects it.
2. Constant evaluation accepts it.

## Answer

Yes; the behavior is undefined, so constant evaluation rejects it (option 1). Clang and EDG agree, and the same delete with a virtual
destructor is accepted by all compilers.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) supports this answer with no doubt, and VIRT-P3 gives the same answer: without a virtual destructor, a `delete` through a base pointer is undefined, so constant evaluation rejects it. GCC 16 still differs. Tidal records GCC 16's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for GCC 16.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on classes and statements (`reports/review/2026-11-03-gap-review-questions-classes-statements.md`) agrees with option 1. A single-object `delete` through a base without a virtual destructor is undefined when the static and dynamic types differ.
