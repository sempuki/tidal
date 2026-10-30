# Q-0021: Is a standard attribute applied to the wrong kind of entity rejected or ignored?

**Status:** provisional
**Affects:** ATTR-2, `tests/declarations/attributes/noreturn-on-variable.cpp`

## Context

`[[noreturn]] int x;` applies an attribute meant for functions to a
variable. Clang 22 and 18 and EDG 7.0 reject it. GCC 16 and 14 accept it,
warning "'noreturn' attribute ignored", even under `-pedantic-errors`.

## Evidence

`tests/RESULTS.md`, row `noreturn-on-variable.cpp`.

## Options

1. Rejected. GCC's acceptance is an extension.
2. Ignored, with a recommended warning.

## Provisional answer

Option 1. Two of three implementations reject it, and other misplacements
(`[[fallthrough]]` outside a switch, arguments to `noreturn`) are rejected
by all three.

## Opinions
