# Q-0178: Is a call through a function pointer whose type differs from the function only in `noexcept` undefined?

**Status:** provisional
**Affects:** PFX-19, CAST-6

## Context

`reinterpret_cast` converts a pointer to a potentially throwing function
into a pointer to a non-throwing function type (CAST-6). PFX-19 makes a
call through a pointer of another function type undefined. The question
is whether a difference in `noexcept` alone makes the call undefined.

The Clang sanitizer (`-fsanitize=function`) stops a call through a
pointer whose parameter types differ from those of the function. The
sanitizer does not stop a call whose types differ only in `noexcept`.
GCC has no such check. Without a check, every compiler in the matrix
runs the function, and the call returns normally when the function does
not throw.

The question covers only a pointer type with `noexcept` that reaches a
function without `noexcept`. The implicit conversion of CONV-10 gives
the other direction, a pointer without `noexcept` to a `noexcept`
function. PFX-19 makes a call through that pointer valid.

## Evidence

Probes on the host Clang and GCC and on the Clang 18, Clang 22, GCC 14,
and GCC 16 images. The test
`tests/expressions/postfix/call-wrong-function-type-sanitizer.cpp`
shows the check for a parameter type. No test covers the `noexcept`
case, because no implementation observes it.
`tests/expressions/postfix/call-noexcept-function-through-plain-pointer.cpp`
runs a call of a `noexcept` function through a pointer without
`noexcept`, with the undefined sanitizer on.

## Options

1. The call is undefined. The `noexcept` specifier is part of the type
   (EXS-2), so the types differ. A non-throwing function type then
   always describes a function that does not throw.
2. The call is defined when the function does not throw, and the
   program ends when the function throws, as at any `noexcept`
   boundary.

## Provisional answer

Yes, undefined (option 1). EXS-2 treats the specifier as part of the type, and an
undefined rule allows the result of every implementation in the matrix.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
