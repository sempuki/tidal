# Q-0258: Can a virtual function have a trailing requires clause?

**Status:** settled
**Affects:** CPT-15, `tests/templates/concepts/virtual.cpp`, `tests/templates/concepts/virtual-override.cpp`

## Context

A member function of a class template has a trailing requires clause
and is virtual. In the second test, the function has no `virtual`, but
it overrides a virtual function of a base.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects: virtual function cannot have a requires clause |
| GCC 14, 16, host | accepts both |
| EDG 7.0 | rejects: a virtual function cannot have a trailing requires clause |

## Evidence

The two tests above. Clang's diagnostic list has a separate error for this case (clang-diag-sema).

## Options

1. Not valid (Clang, EDG). A call through a base class could reach a
   function whose constraints are not satisfied.
2. Valid (GCC).

## Answer

No; Clang and EDG agree, GCC differs (option 1). Two of three implementations agree, and a virtual call has no point at which the constraints could be checked.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) supports this answer with no doubt, and CPT-P5 gives the same answer: a clause on a virtual function is not valid, because a call through a base class does not check the constraints. GCC 16 still differs. Tidal records GCC 16's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for GCC 16.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) agrees with option 1. A trailing requires clause on any virtual function is forbidden. A function that overrides a virtual function is virtual, with or without the keyword, so the second test falls under the same ban. GCC's acceptance is a defect.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
