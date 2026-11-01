# Q-0074: Is language linkage part of a function type?

**Status:** provisional
**Affects:** LSP-6, `tests/declarations/linkage-specifications/c-function-pointer-assign.cpp`, `tests/declarations/linkage-specifications/function-type-linkage-overload.cpp`

## Context

A function declared `extern "C"` and a function with C++ linkage can have
the same parameters and return type. EDG 7.0 treats their types as
different, so a pointer to the C function does not initialize a pointer
to a C++ function type. Clang and GCC treat the types as the same. All
three accept two declarations that overload on the two types.

## Options

1. It is implementation-defined if language linkage is part of a function
   type.
2. Language linkage is part of the function type.
3. Language linkage is not part of the function type.

## Provisional answer

Implementation-defined (option 1). Two implementations ignore language linkage in the type and
one keeps it, and programs need to work with both.

## Opinions
