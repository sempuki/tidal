# Q-0350: Does Clang conform when it accepts a class whose size it cannot represent?

**Status:** provisional
**Affects:** STG-1, TR-15, `tests/foundations/storage/class-size-exceeds-size-type-ilp32.cpp`, `tests/foundations/translation/limit-class-wrong-size-clang-18-lp64.cpp`, `tests/foundations/translation/limit-class-largest-gcc-lp64.cpp`, `tests/foundations/translation/limit-class-largest-edg-lp64.cpp`, `tests/foundations/translation/limit-class-exceeded-gcc-lp64.cpp`

## Context

Two cases give a class whose `sizeof` in Clang differs from the sum of
its members.

| Case | Clang 22 | Clang 18 | GCC 16, 14 | EDG 7.0 |
|---|---|---|---|---|
| x86-64, a class of 2<sup>61</sup> bytes or more | rejects: the structure is too large | accepts, and `sizeof` gives a wrong value | accepts up to 2<sup>63</sup> - 1 bytes | accepts up to 2<sup>64</sup> - 1 bytes |
| `-m32`, a class of 3 * 0x60000000 bytes, more than `std::size_t` holds | accepts, and `sizeof` gives the size modulo 2<sup>32</sup> | the same as Clang 22 | rejects: overflow in a constant expression | rejects: the class is too large |

On x86-64, Clang 22 rejects every class of 2<sup>60</sup> bytes or more,
so its sizes stay right. With `-m32`, Clang 22 sets no such limit
below the range of `std::size_t`. A class of two members of
2<sup>31</sup> bytes each gives `sizeof` the value 0.

## Evidence

The tests in Affects. The x86-64 case of a class larger than
`std::size_t` holds is Q-0308, where Clang 22 rejects and Clang 18
accepts.

## Options

1. A class larger than `std::size_t` holds is not valid (STG-1), and
   the implementation rejects it. Clang 22 and 18 with `-m32` have a
   defect. A class that fits `std::size_t` but passes the
   implementation's limit is rejected under TR-15, so Clang 18 on x86-64
   also has a defect, which Clang 22 fixes.
2. The size of a class past the limit is outside the specification, and
   any result is allowed.

## Provisional answer

Clang has a defect in both cases (option 1). Every other compiler of
the matrix rejects the class or gives the right size, and Q-0308 already
reads STG-1 this way for x86-64.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
