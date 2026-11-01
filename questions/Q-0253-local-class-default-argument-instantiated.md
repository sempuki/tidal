# Q-0253: Is the default argument of a local class's member function instantiated with the enclosing function?

**Status:** provisional
**Affects:** INST-12, `tests/templates/instantiation/local-class-default-argument.cpp`

## Context

A function template declares a local class with a member function
`get(int y = T::nothing)`. No call uses the default argument, and the
function is instantiated with `T = int`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects |
| EDG 7.0 | rejects |
| GCC 14, 16, host | accepts |

Every compiler instantiates the default member initializers, the
exception specifications, and the local enumerations of the function.

## Evidence

The test above and the tests `local-class-member-initializer.cpp`,
`local-class-exception-specification.cpp`, and `local-enumeration.cpp`.
Item 29 of the gap review `reports/review/2026-10-31-gap-review-dependent-names-and-instantiation.md` says that the default
arguments of a local class are instantiated with the function.

## Options

1. The default argument is instantiated with the function (Clang, EDG).
2. The default argument waits for a call that uses it (GCC).

## Provisional answer

Yes; Clang and EDG agree, GCC differs (option 1). Two of three implementations and the gap review agree.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
