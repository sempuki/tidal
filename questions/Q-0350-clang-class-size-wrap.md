# Q-0350: Does Clang conform when it accepts a class whose size it cannot represent?

**Status:** provisional
**Affects:** STG-1, TR-15, `tests/foundations/storage/class-size-exceeds-size-type-ilp32.cpp`, `tests/foundations/translation/limit-class-wrong-size-clang-18-lp64.cpp`, `tests/foundations/translation/limit-class-largest-gcc-lp64.cpp`, `tests/foundations/translation/limit-class-largest-edg-lp64.cpp`, `tests/foundations/translation/limit-class-exceeded-gcc-lp64.cpp`
**Waits for:** a review (doubt)

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
accepts. GCC 16 and the x86-64 build of EDG skip the `-m32` test.
Every compiler rejects a smaller limit too, as TR-14 and TR-15 allow.

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

Kept open on 2026-11-03 under decision 0054, because the two reviews read the reference with doubt, and they differ on whether the reference rejects a class that `std::size_t` cannot count.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-11-03

The source-reader gap review of the object model (`reports/review/2026-11-03-gap-review-object-model.md`) reads the reference as supporting option 1 for the 32-bit case. The reference makes a type ill-formed when its object representation needs more bytes than `std::size_t` can count. Clang accepts such a class and gives a reduced size, which conflicts with the reference. The 64-bit case of Clang 18 is a class that fits `std::size_t` and passes Clang's own limit. The reference asks for correct execution only within the implementation's resource limits, and says nothing about a program past them. A diagnostic is the better behavior there, but the reference does not plainly require it.

### Gap review, 2026-11-03 (program structure)

The source-reader gap review of program structure (`reports/review/2026-11-03-gap-review-program-structure.md`) reads the reference as silent. The review finds no rule that rejects a class whose size `std::size_t` cannot hold. The reference asks an implementation to run every valid program correctly within its resource limits, and to document those limits where they are known. A wrong result from `sizeof` is not a correct execution, so Clang conforms only if the class is past one of its limits, and Clang documents no such limit. Option 1 relies on STG-1, which goes beyond the reference by this reading.
