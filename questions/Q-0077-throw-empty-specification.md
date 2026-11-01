# Q-0077: Is throw() a valid exception specification?

**Status:** provisional
**Affects:** EXS-5, `tests/declarations/exception-specifications/throw-empty-removed.cpp`

## Context

`void f() throw();` uses the old form of an exception specification. Clang and GCC accept the form under `-pedantic-errors` and treat it as `noexcept`. EDG 7.0 rejects it. All three reject `throw(int)`.

GCC 14 and 16 give the same result, and so do Clang 18 and 22. The host
GCC 16.2 and Clang 22.1 agree with them. No newer version settles the
split.

## Options

1. It is implementation-defined if `throw()` is accepted. If `throw()` is accepted, `throw()` means `noexcept`.
2. `throw()` is not valid.
3. `throw()` is valid and means `noexcept`.

## Provisional answer

Option 1. Two implementations accept `throw()` and one rejects it.

## Opinions
