# Q-0012: Should Tidal set a minimum for the largest array or class an implementation accepts?

**Status:** settled
**Affects:** TR-14, TR-15, `tests/declarations/declarators/array-too-large.cpp`, `tests/declarations/declarators/array-4gib-ilp32.cpp`, `tests/foundations/translation/limit-array-*.cpp`, `tests/foundations/translation/limit-class-*.cpp`

## Context

Each compiler of the matrix has a largest size for an array type and a
largest size for a class type. The sizes differ between compilers and
between targets. The figures are sizes in bytes.

| Compiler | Array, x86-64 | Array, `-m32` | Class, x86-64 | Class, `-m32` |
|---|---|---|---|---|
| Clang 22 | 2<sup>61</sup> - 1 | 2<sup>32</sup> - 1 | 2<sup>60</sup> - 1 | 2<sup>32</sup> - 1 |
| Clang 18 | 2<sup>61</sup> - 1 | 2<sup>32</sup> - 1 | 2<sup>61</sup> - 1 | 2<sup>32</sup> - 1 |
| GCC 16, 14 | 2<sup>63</sup> - 1 | 2<sup>31</sup> - 1 | 2<sup>63</sup> - 1 | 2<sup>31</sup> - 1 |
| EDG 7.0 | 2<sup>64</sup> - 1 | 2<sup>32</sup> - 1 | 2<sup>64</sup> - 1 | 2<sup>32</sup> - 1 |

Clang 18 rejects no class, and Clang 22 rejects no class with `-m32`.
Above the figure of the table, these compilers give `sizeof` a wrong
value (Q-0350). Each compiler rejects an array
type past its limit, also in an alias declaration. Each compiler
rejects a class past its limit only at a use that needs the size, such
as an object definition or `sizeof`.

## Evidence

`tests/RESULTS.md`, rows `array-too-large.cpp`, `array-4gib-lp64.cpp`,
`array-4gib-ilp32.cpp`, and the `limit-array-*` and `limit-class-*`
tests of `tests/foundations/translation/`. Each test declares one array
or one class at a figure of the table, or one byte past it.

For an array, GCC's message says that the size "exceeds maximum object
size", and Clang's says "array is too large". For a class, Clang 22
says "structure is too large", GCC says "size of type is too large",
and EDG says "class is too large".

## Options

1. Leave each limit to each implementation (`impl-defined`), with only
   the requirement that the size fits the type of `sizeof` (STG-1).
2. Require a minimum that every implementation must accept, so programs
   below it are portable.

## Answer

No; each limit is implementation-defined (option 1). Implementations
disagree on the limits, and the limits change with the target.

Settled on 2026-11-03 under decision 0054. The review supports this answer with no doubt, and TR-P6 gives the same answer: each implementation sets its own limits, and Tidal sets no minimum. Each family's limit is an implementation-defined choice, so no family differs.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of program structure (`reports/review/2026-11-03-gap-review-program-structure.md`) reads the reference as supporting option 1, the provisional answer, with no doubt. The reference recommends a minimum object size and says that its recommended minimums do not decide conformance.
