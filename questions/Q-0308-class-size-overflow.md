# Q-0308: Must an implementation reject a class whose size does not fit in `std::size_t`?

**Status:** settled
**Affects:** STG-1, `tests/foundations/storage/class-size-exceeds-size-type.cpp`

## Context

A class has 33 members of 2<sup>59</sup> bytes each, so its size
exceeds the largest value of `std::size_t` on x86-64. The test takes
`sizeof` of the class in a constant.

| Compiler | Result |
|---|---|
| Clang 22, host 22 | accepts the class definition alone, and at `sizeof` rejects the class: the structure is too large |
| EDG 7.0 | rejects the class: the class is too large |
| GCC 14, 16, host 16 | accepts the class definition, and rejects `sizeof` with an overflow in a constant expression |
| Clang 18 | accepts, and `sizeof` gives the size reduced modulo 2<sup>64</sup> |

## Evidence

The test above. An array whose size does not fit is rejected by every
compiler (`array-size-exceeds-size-type.cpp`, DCL-10).

## Options

1. The program is invalid, and an implementation rejects it (STG-1).
   Clang 18 has a defect that Clang 22 fixes.
2. The size limit for classes is an implementation limit with no
   required diagnostic.

## Answer

Yes (STG-1); Clang 18 accepts (option 1). Every current compiler rejects the program, and the reduced
size of Clang 18 is not the size of the class.

Settled on 2026-11-01 under decision 0051, because the newest version of each baseline family (Clang 22, GCC 16, EDG 7.0) agrees with this answer. Clang 18 still accepts the class, and the test keeps its override for Clang 18. GCC 16 rejects the program at `sizeof` and accepts the class definition.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-11-01

The source-reader gap review of question cluster 1 (`reports/review/2026-11-01-gap-review-question-cluster-1.md`) supports the answer. Forming a type whose object representation is larger than the largest `size_t` value is ill-formed, classes included, so a compiler should diagnose the class definition even when the program never takes its size.
