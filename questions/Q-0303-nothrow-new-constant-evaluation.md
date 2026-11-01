# Q-0303: Can constant evaluation use a `new` expression with the nothrow tag?

**Status:** provisional
**Affects:** NEW-37, `tests/expressions/new-delete/constant-evaluation-nothrow.cpp`

## Context

Constant evaluation can use a `new` expression that calls a global
allocation function of the implementation. The nothrow forms are global
allocation functions that the library defines. A program that includes
no header declares them itself (LSUP-6). Clang 18 and 22 and GCC 16
evaluate `new (std::nothrow) int(4)` during constant evaluation. GCC 14
says the call is to a function that is not `constexpr`. EDG treats the
expression as a placement `new` and rejects it.

## Evidence

`tests/expressions/new-delete/constant-evaluation-nothrow.cpp`.

## Options

1. Constant evaluation accepts the nothrow forms. GCC 14 and EDG
   differ.
2. Constant evaluation rejects every `new` expression with placement
   arguments, the nothrow tag included. Clang and GCC 16 differ.

## Provisional answer

Yes (NEW-37); Clang and GCC 16 accept, GCC 14 and EDG reject (option 1). The newest Clang and GCC agree, and the nothrow forms are
global allocation functions of the implementation, as the forms
without placement arguments are.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
