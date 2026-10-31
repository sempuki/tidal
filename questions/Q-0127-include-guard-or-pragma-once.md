# Q-0127: Which mechanism does Tidal specify for including a header only once?

**Status:** provisional
**Affects:** INC-4, INC-5, `tests/foundations/odr/guarded-header-twice.cpp`, `tests/foundations/odr/pragma-once-header-twice.cpp`, `tests/foundations/odr/header-class-shared.cpp`

## Context

A header that defines a class, an enumeration, or an inline entity must
appear once in each translation unit, or the unit defines the entity
twice (ODR-1). Programs use one of two mechanisms:

| Mechanism | Text | Matrix compilers |
|---|---|---|
| Include guard | `#ifndef NAME`, `#define NAME`, the header's text, `#endif` | Clang, GCC, and EDG skip the second inclusion |
| `#pragma once` | `#pragma once` as the first line | Clang, GCC, and EDG skip the second inclusion, also under `-pedantic-errors` and `--strict` |

The include guard uses three preprocessing directives, which the
charter otherwise keeps out of scope. `#pragma once` is a pragma, and
the effect of a pragma is up to each implementation.

## Evidence

`guarded-header-twice.cpp` and `pragma-once-header-twice.cpp` run on
every compiler in the matrix. `header-included-twice.cpp` shows that a
header with neither mechanism is rejected when it is included twice.

## Options

1. Specify only the include guard. The guard works on every
   implementation, and Tidal states only the three directives in the
   form that the guard needs.
2. Specify only `#pragma once`. The form is shorter, but Tidal would
   require a behavior that each implementation chooses.
3. Specify the include guard as a `required` rule and `#pragma once` as
   an `impl-defined` rule with the matrix compilers' behavior.

## Provisional answer

Option 3. Programs in the wild use both forms, and the tests show the
same behavior for both on the matrix. The guard stays the portable form,
so the guard is the form in `required` text.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
