# Q-0021: Is a standard attribute applied to the wrong kind of entity rejected or ignored?

**Status:** provisional
**Affects:** ATTR-2, `tests/declarations/attributes/noreturn-on-variable.cpp`, `nodiscard-on-variable.cpp`, `no-unique-address-bitfield.cpp`

## Context

`[[noreturn]] int x;` applies an attribute meant for functions to a
variable. Two other standard attributes split the compilers in the same
way. The compilers that accept a program ignore the attribute with a
warning, even under `-pedantic-errors`.

| Program | Clang 18, 22 | GCC 14, 16 | EDG 7.0 |
|---|---|---|---|
| `[[noreturn]]` on a variable | rejects | accepts, "'noreturn' attribute ignored" | rejects |
| `[[nodiscard]]` on a variable | accepts, "only applies to ... functions" | accepts, "can only be applied to functions" | rejects |
| `[[no_unique_address]]` on a bit-field | rejects | accepts, "cannot be applied to a bit-field" | rejects |

The host GCC 16.2 and Clang 22.1 give the same results. No newer
version of any compiler settles a row.

## Evidence

The three tests above. Each has an override for the compilers that
accept the program.

## Options

1. Rejected. GCC's acceptance is an extension.
2. Ignored, with a recommended warning.

## Provisional answer

Rejected (option 1). Two of three implementations reject `noreturn` on a variable
and `no_unique_address` on a bit-field. Other misplacements
(`[[fallthrough]]` outside a switch, arguments to `noreturn`) are
rejected by all three. For `nodiscard` on a variable, only EDG rejects,
so option 1 has the weakest support there. The specification follows
option 1 for all three, so that one rule covers every misplaced
standard attribute.

## Opinions
