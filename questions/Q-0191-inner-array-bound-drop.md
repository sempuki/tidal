# Q-0191: Does an array bound dropped at an inner pointer level need `const` above it?

**Status:** provisional
**Affects:** CV-10, `tests/declarations/declarators/qualification-conversion-array-inner-bound.cpp`

## Context

`int (**)[3]` converts to `int (**)[]`. The bound is dropped at the
second level, and the first level has no `const`. With `const` at the
first level, every compiler accepts the conversion.

| Compiler | `int (**)[3]` to `int (**)[]` |
|---|---|
| Clang 18, Clang 22 | rejected |
| GCC 14, GCC 16 | accepted |
| EDG 7.0 | accepted |

Clang treats the dropped bound as it treats added `const`. Through the
converted pointer, a program can store a pointer to an array of another
bound in the original `int (*)[3]`.

## Evidence

`tests/declarations/declarators/qualification-conversion-array-inner-bound.cpp`.

## Options

1. A dropped bound needs no `const` at the levels above (GCC, EDG).
2. A dropped bound needs `const` at each level above, except the
   outermost (Clang).

## Provisional answer

Option 1. Two of three implementations agree.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
