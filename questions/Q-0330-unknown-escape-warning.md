# Q-0330: Must an implementation reject a character literal with an unknown escape sequence?

**Status:** provisional
**Affects:** PRIM-18, `tests/expressions/primary/char-unknown-escape.cpp`

## Context

PRIM-18 is a `reject` rule, and one of its cases is an unknown escape
sequence such as `'\q'`. The compilers in the matrix all give a
diagnostic for `'\q'`, but only some of them stop the translation.

- Clang 18 and Clang 22 warn under `-Wunknown-escape-sequence`. Under
  `-pedantic-errors` the warning is an error.
- GCC 14 and GCC 16 warn. Under `-pedantic-errors` the warning is an
  error.
- EDG 7.0 in strict mode warns "unrecognized character escape" and
  translates the program.

The test uses no `-Werror`, because EDG skips each test that uses
`-Werror`.

## Evidence

`tests/expressions/primary/char-unknown-escape.cpp`, without
`-Werror`. Clang and GCC reject the program. EDG accepts it with a
warning.

## Options

1. The program is invalid and an implementation rejects it. EDG differs
   in strict mode.
2. The program is invalid, and a warning is enough. PRIM-18 then needs
   a separate `no-diagnostic` or `recommended` rule for unknown escapes.
3. An unknown escape is conditionally supported, with a meaning that
   the implementation documents.

## Provisional answer

The program is invalid and is rejected (option 1).
Clang and GCC reject the program in their strict modes, and an
unknown escape has no meaning that a program can rely on. EDG gives a
diagnostic but translates the program.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
