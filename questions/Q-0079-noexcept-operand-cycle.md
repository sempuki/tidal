# Q-0079: Must an implementation diagnose a noexcept operand that depends on itself?

**Status:** provisional
**Affects:** EXS-6, `tests/declarations/exception-specifications/operand-cycle.cpp`

## Context

A member function `f` has the specifier `noexcept(noexcept(f()))`. The
value of the specifier depends on itself. Clang rejects a use of
`noexcept(S{}.f())`. GCC and EDG accept it.

## Options

1. The program is not valid, and an implementation need not diagnose it
   (`no-diagnostic`).
2. Implementations must reject the program.
3. The specifier has a fixed value, such as `false`.

## Provisional answer

Option 1. Clang diagnoses the program. GCC and EDG still conform.

## Opinions
