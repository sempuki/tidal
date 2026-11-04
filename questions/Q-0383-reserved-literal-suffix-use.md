# Q-0383: Does a literal call a literal operator whose suffix does not start with `_`?

**Status:** provisional
**Affects:** OPR-8, OPR-9, `tests/classes/operators/literal-reserved-suffix-used.cpp`, `tests/classes/operators/literal-reserved-suffix-u8.cpp`
**Source:** finding X19-1 of `reports/clean-room/X19/report.md`

## Context

OPR-9 says that a literal suffix that does not start with `_` is
reserved, and that an implementation should warn about a literal
operator with such a suffix. A program can still declare the operator.
The question is whether a literal with that suffix then calls the
operator.

| Program | Clang 22 | GCC 16 | EDG 7.0 |
|---|---|---|---|
| `operator"" km` and the literal `1km` | rejects the literal | calls the operator | calls the operator |
| `operator"" u8` and the literal `1u8` | rejects the literal | calls the operator | rejects the literal |

Clang warns at the declaration that no literal will call the operator.
GCC and EDG warn that the suffix is reserved. EDG reads `1u8` as the
`u` suffix followed by extra text.

A suffix that is a form in the syntax of literals, such as `u` or `f`,
keeps its built-in meaning on every compiler
(`tests/classes/operators/literal-builtin-suffix-wins.cpp`).

## Evidence

The two tests in Affects. Each runs and exits 0 on GCC. Clang rejects
both, and EDG rejects the `u8` test.

## Options

1. The literal calls the literal operator (GCC, and EDG for `km`). The
   reservation only asks for a warning, as OPR-9 says.
2. The literal is not valid (Clang). A reserved suffix never matches a
   literal operator.

## Provisional answer

Yes; the literal calls the literal operator (option 1). OPR-8 and
PRIM-16 already read this way, and two of three families call the
operator for a suffix such as `km`. EDG's rejection of `1u8` is a
separate reading of the `u` suffix, which the tests record.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
