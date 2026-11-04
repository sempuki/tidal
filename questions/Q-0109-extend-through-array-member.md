# Q-0109: Does a reference bound to an element of an array member of a temporary extend the temporary?

**Status:** settled
**Affects:** REF-4, `tests/declarations/initialization/references/extend-through-array-member-subscript.cpp`

## Context

An rvalue reference binds to one element of an array member of a class prvalue. Clang and GCC accept the binding and keep the temporary alive with the reference. EDG 7.0 rejects the binding as an rvalue reference to an lvalue.

## Options

1. The binding is valid and extends the temporary.

2. The binding is not valid.

## Answer

Yes; the binding is valid and extends the temporary (option 1). Clang and GCC agree, and REF-4 already covers a member of the temporary.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on overloading, references, and list initialization (`reports/review/2026-11-03-gap-review-questions-overloading.md`) supports this answer with no doubt, and REF-P2 gives the same answer: the extension reaches bindings through a member and a subscript. EDG 7.0 still differs. Tidal records EDG 7.0's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for EDG 7.0.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on overloading, references, and list initialization (`reports/review/2026-11-03-gap-review-questions-overloading.md`) agrees with option 1. Member access and subscripting of an expiring object give an expiring result, so an rvalue reference can bind. The forms that carry the lifetime extension include a subscript of an array operand that carries it and a member access with a dot. EDG's rejection is a defect.

