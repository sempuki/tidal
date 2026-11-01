# Q-0197: Must an implementation diagnose another declaration of a friend function template that gives a default?

**Status:** provisional
**Affects:** TPL-38

## Context

A class defines `template <class T = int> friend int pick(Host)`.
Another declaration of the same function template comes before or after
the class.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | accepts |
| GCC 14, 16, host | accepts |
| EDG 7.0 | rejects: a friend template declaration with a default must be the only declaration |

All compilers accept the friend definition when it is the only
declaration.

## Evidence

`tests/templates/declarations/default-friend-redeclared.cpp`, with an
override for EDG, and `default-friend-definition.cpp`.

## Options

1. The other declaration makes the program invalid, and an
   implementation need not diagnose it. All three compilers conform.
2. An implementation must reject the program (EDG).
3. The program is valid (Clang, GCC).

## Provisional answer

Option 1. EDG diagnoses the case, and Clang and GCC give the program a
meaning. A rule without a required diagnostic covers both.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
