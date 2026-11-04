# Q-0106: Can a parameter have a reference to a function type with a ref-qualifier?

**Status:** settled
**Affects:** DCL-16, `tests/declarations/declarators/qualified-function-type-parameter.cpp`

## Context

A type alias names a function type with a ref-qualifier. A parameter is declared as a reference to that type. Clang and GCC reject the declaration. EDG 7.0 accepts it.

## Options

1. Not valid, as for a pointer to such a type.

2. Valid.

## Answer

No, as for a pointer to such a type (option 1). Clang and GCC agree, and a pointer to the same type is rejected by all three.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) supports this answer with no doubt, and DCL-P3 gives the same answer: such a type can be named and used for member functions and pointers to members, and for nothing else. EDG 7.0 still differs. Tidal records EDG 7.0's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for EDG 7.0.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. A function type with a cv-qualifier or a ref-qualifier can appear only in a short list of places: a non-static member function, the target of a pointer to member, a function alias, and two template cases. A reference to such a type is not on the list, as a plain pointer is not.
