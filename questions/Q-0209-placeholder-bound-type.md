# Q-0209: What type does a placeholder constant parameter get from an array bound?

**Status:** provisional
**Affects:** DED-11, `tests/templates/deduction/match-bound-type.cpp`

## Context

A function template has `template <auto N>` and takes `int (&)[N]`. A
call passes an array of three `int`. `N` is 3 on every compiler. The
type of `N` differs.

| Compiler | Type of `N` |
|---|---|
| Clang 18, 22, host | the type of a `sizeof` expression |
| EDG 7.0 | the type of a `sizeof` expression |
| GCC 14, 16, host | `int` |

## Evidence

`tests/templates/deduction/match-bound-type.cpp`, with an override for
GCC. The sanitized Clang source (clang-sema-deduction) deduces a bound
from a braced list with the size type. Item E2 of the deduction
inventory describes the behavior.

## Options

1. The bound has the type of a `sizeof` expression (Clang, EDG).
2. The bound has the type `int` (GCC).

## Provisional answer

Option 1. Two of three implementations and the inventory agree, and the
number of elements of an array has the type of `sizeof` elsewhere.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
