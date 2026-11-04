# Q-0358: Does a `new` expression use the deallocation function when its constructor is `noexcept`?

**Status:** provisional
**Affects:** ODR-5, ODR-8, `tests/foundations/odr/new-noexcept-constructor-uses-deallocation.cpp`

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
`tests/foundations/odr/new-noexcept-constructor-uses-deallocation.cpp`.
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

Yes; the `new` expression uses the deallocation function whatever the exception specification of the constructor (option 2). The program of the test is not valid, and ODR-8 needs no diagnostic. The test expects the link failure of GCC 16 at `-O0`, and records the run of Clang, GCC 14, and GCC 16 at `-O2` with overrides.

Changed on 2026-11-03. The source-reader gap review of program structure (`reports/review/2026-11-03-gap-review-program-structure.md`) reads the reference as supporting option 2. The reading matches GCC 16 at `-O0`, and Tidal's own rules give the same answer. ODR-P1 says that a `new` expression uses its allocation and deallocation functions, with no condition. ODR-5 now drops the condition on the constructor, and adds the uses by the definitions of a constructor and a destructor. The test is now `new-noexcept-constructor-uses-deallocation.cpp`.

Kept open on 2026-11-03 under decision 0054, because ODR-5 changed on the same day, and a question settles only after a reader or a review has used the changed rule (decision 0055). Every compiler's result fits ODR-8.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-11-03

The source-reader gap review of program structure (`reports/review/2026-11-03-gap-review-program-structure.md`) reads the reference as supporting option 2. A `new` expression names the matching deallocation function whenever lookup finds one match, and the exception specification of the constructor plays no part. Separately, the definition of any constructor of a class uses the allocation and deallocation functions of the class that are not placement forms. So the program of the test uses a function that it never defines. ODR-8 needs no diagnostic, so a link failure and a run are both allowed, and GCC 16 at `-O0` conforms.
