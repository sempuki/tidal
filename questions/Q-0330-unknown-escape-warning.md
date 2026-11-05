# Q-0330: Must an implementation reject a character literal with an unknown escape sequence?

**Status:** provisional
**Affects:** PRIM-18, PRIM-35, `tests/expressions/primary/char-unknown-escape.cpp`, `tests/expressions/primary/string-unknown-escape.cpp`, `tests/expressions/primary/char-unknown-escape-outside-basic.cpp`
**Waits for:** the maintainer (pending.md row 4)

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

An unknown escape of a character of the basic character set is conditionally supported, and an escape of any other character is rejected (option 3). Clang and GCC reject `'\q'` in their strict modes, and EDG accepts it with a warning, so each conforms.

Changed on 2026-11-03 to option 3. The review reads option 3, every compiler conforms to it, and PRIM-P4 gives the same answer: the parts of a literal that depend on the character set belong to the implementation. A probe on the matrix shows that EDG accepts `'\q'`, `"\q"`, `'\@'`, `'\$'`, and a backtick escape, and that every compiler rejects a backslash before `é`. The new `conditionally-supported` rule PRIM-35 covers the escape, with the tests `char-unknown-escape.cpp` and `string-unknown-escape.cpp`. PRIM-18 keeps a backslash before a character outside the basic character set, with the new test `char-unknown-escape-outside-basic.cpp`. The question stays open, because the rule changed on the same day.

Kept open on 2026-11-04 under decision 0054, because the gap review of the rules that phase 7 added reads the string literal case with doubt, and the question waits on the maintainer (`documentation/decisions/pending.md`, row 4).

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) reads option 3. A backslash before a member of the basic character set that no other escape uses forms an escape of a third kind. Such an escape is conditionally supported, with an implementation-defined value, in a character literal and in a string literal. An implementation that does not support it must diagnose the program. EDG's warning with a translated program, and the errors of Clang and GCC under their strict options, all conform. A backslash before a character outside the basic character set fits no escape form, and that program is not valid.

### Gap review, 2026-11-04

The source-reader gap review of the rules that phase 7 added (`reports/review/2026-11-04-gap-review-phase-7-additions.md`) agrees for a character literal and reads the string literal case with doubt. The set of characters matches the reference, and a character literal of one such escape is conditionally supported with an implementation-defined value. For a string literal, the reference says only that each such escape gives an implementation-defined sequence of code units, and does not call it conditionally supported. In that reading EDG's acceptance of `"\q"` is required, and the rejections of Clang 22 and GCC 16 are not. Tidal keeps PRIM-35 for both literals (`documentation/decisions/pending.md`, row 4).

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
