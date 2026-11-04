# Q-0079: Must an implementation diagnose a noexcept operand that depends on itself?

**Status:** provisional
**Affects:** EXS-6, `tests/declarations/exception-specifications/operand-cycle.cpp`

## Context

A member function `f` has the specifier `noexcept(noexcept(f()))`. The
value of the specifier depends on itself. Clang rejects a use of
`noexcept(S{}.f())`. GCC and EDG accept it.

GCC 14 and 16 give the same result, and so do Clang 18 and 22. The host
GCC 16.2 and Clang 22.1 agree with them. No newer version settles the
split.

## Options

1. The program is not valid, and an implementation need not diagnose it
   (`no-diagnostic`).
2. Implementations must reject the program.
3. The specifier has a fixed value, such as `false`.

## Provisional answer

No; the program is not valid, and the error is `no-diagnostic` (option 1). Clang diagnoses the program. GCC and EDG still conform.

The test expects acceptance, the outcome of an implementation that does
not detect the dependency, and records Clang's rejection as an override.

Kept open on 2026-11-03 under decision 0054, because the review finds the reference silent.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) finds the reference silent. An exception specification is needed for a function that overload resolution picks in an expression, and the operand of the member's own specifier calls the member, so the specification needs itself. No rule covers a specification that depends on itself. No rule makes such a program invalid, with or without a diagnostic, and no rule fixes the specification's value. The deferred handling of these specifiers covers template specializations, and the limit on recursive instantiation does not reach a member of an ordinary class. Option 1 is a reasonable reading of the gap, and Clang's error is consistent with it.
