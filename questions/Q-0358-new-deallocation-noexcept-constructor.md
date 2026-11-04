# Q-0358: Does a `new` expression use the deallocation function when its constructor is `noexcept`?

**Status:** provisional
**Affects:** ODR-5, ODR-8, `tests/foundations/odr/new-noexcept-constructor-no-deallocation.cpp`

## Context

A class declares its own `operator new` and `operator delete`, and
defines only `operator new`. A `new` expression creates an object of
the class. ODR-5 says that the `new` expression uses the deallocation
function if the constructor can throw. A constructor without
`noexcept` can throw (EXS-4).

| Compiler | Constructor without `noexcept` | `noexcept` constructor |
|---|---|---|
| Clang 18, Clang 22, and Clang 22 at `-O2` | fails to link | links and runs |
| GCC 14 | fails to link | links and runs |
| GCC 16 at `-O0` | fails to link | fails to link |
| GCC 16 at `-O2` | fails to link | links and runs |
| EDG 7.0 | builds no program | builds no program |

## Evidence

`tests/foundations/odr/new-uses-deallocation-function.cpp` and
`tests/foundations/odr/new-noexcept-constructor-no-deallocation.cpp`.
Host probes by the clean-room writer: Clang links the `noexcept` case,
and a constructor that only calls a `noexcept` function, but is not
itself `noexcept`, fails to link on Clang and GCC.

## Options

1. The deallocation function is used only when the constructor is not
   non-throwing (EXS-4). GCC 16 at `-O0` refers to a function that the
   program does not use.
2. A `new` expression with a class allocation function always uses the
   matching deallocation function.

## Provisional answer

No; a `noexcept` constructor leaves the deallocation function unused (option 1). Clang and GCC 14 agree, and ODR-5 ties the use to the
constructor's exception specification.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
