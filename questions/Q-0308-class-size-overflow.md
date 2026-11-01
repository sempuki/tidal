# Q-0308: Must an implementation reject a class whose size does not fit in `std::size_t`?

**Status:** compiler defect
**Affects:** STG-1, `tests/foundations/storage/class-size-exceeds-size-type.cpp`

## Context

A class has 33 members of 2<sup>59</sup> bytes each, so its size
exceeds the largest value of `std::size_t` on x86-64. The test takes
`sizeof` of the class in a constant.

| Compiler | Result |
|---|---|
| Clang 22, host 22 | rejects the class: the structure is too large |
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

## Provisional answer

Yes (STG-1); Clang 18 accepts (option 1). Every current compiler rejects the program, and the reduced
size of Clang 18 is not the size of the class.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
