# Q-0077: Is throw() a valid exception specification?

**Status:** provisional
**Affects:** EXS-5, `tests/declarations/exception-specifications/throw-empty-removed.cpp`

## Context

`void f() throw();` uses the old form of an exception specification. Clang and GCC accept it under `-pedantic-errors` and treat it as `noexcept`. EDG 7.0 rejects it. All three reject `throw(int)`.

## Options

1. It is implementation-defined if `throw()` is accepted. If it is, it means `noexcept`.
2. `throw()` is not valid.
3. `throw()` is valid and means `noexcept`.

## Provisional answer

Option 1. Two implementations accept it and one rejects it.

## Opinions
