# Q-0077: Is throw() a valid exception specification?

**Status:** provisional
**Affects:** EXS-5, `tests/declarations/exception-specifications/throw-empty-removed.cpp`

## Context

`void f() throw();` uses the old form of an exception specification. Clang and GCC accept the form under `-pedantic-errors` and treat it as `noexcept`. EDG 7.0 rejects it. All three reject `throw(int)`.

## Options

1. It is implementation-defined if `throw()` is accepted. If `throw()` is accepted, `throw()` means `noexcept`.
2. `throw()` is not valid.
3. `throw()` is valid and means `noexcept`.

## Provisional answer

Option 1. Two implementations accept `throw()` and one rejects it.

## Opinions
