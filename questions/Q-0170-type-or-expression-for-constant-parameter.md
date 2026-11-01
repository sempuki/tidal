# Q-0170: Is `int()` a type or a value when the parameter is a constant parameter?

**Status:** provisional
**Affects:** TARG-7, TARG-8

## Context

`template <int N> struct Num;` and the use `Num<int()>`. The argument
`int()` can be read as a function type or as a value-initialized `int`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects: the argument is the function type `int ()` |
| GCC 14, 16, host | rejects: a type for a constant parameter |
| EDG 7.0 | accepts, with the value `0` |

## Evidence

`tests/templates/arguments/type-or-expression-constant.cpp`, with an
override for EDG. `tests/templates/arguments/type-or-expression.cpp`
shows that all compilers read `int()` as a type for a type parameter.

## Options

1. The type reading wins in every template argument, and the argument
   is not valid for a constant parameter (Clang, GCC).
2. The kind of the parameter decides the reading (EDG).

## Provisional answer

Option 1. Clang and GCC agree, and the reading of an argument then does
not depend on the template's parameters.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
