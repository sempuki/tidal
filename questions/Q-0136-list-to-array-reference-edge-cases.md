# Q-0136: Do empty braces or a designated list have a sequence to a reference to an array?

**Status:** provisional
**Affects:** ICS-17, ICS-18
**Parent:** Q-0294, the model of braced lists for references, arrays, and conversion sequences

## Context

The lists grid on the page shows two cases where the compilers differ.

Empty braces `{}` and a parameter `int (&&)[]`, an array of unknown
bound:

| Compiler | Result |
|---|---|
| GCC 14, GCC 16 | a sequence exists, and it beats `int (&&)[2]` (fewer elements) and `Point` |
| Clang 18, Clang 22 | no sequence |
| EDG 7.0 | a sequence exists, and it ties with `int` and with `int (&&)[2]` |

A designated list `{.x = 1}` and a parameter `int (&&)[2]` or
`int (&&)[]`, against an aggregate `Point` with a member `x`:

| Compiler | Result |
|---|---|
| Clang 18, Clang 22 | no sequence to the array, so `Point` is chosen |
| GCC 14, GCC 16 | rejects ("name 'x' used in a GNU-style designated initializer for an array") |
| EDG 7.0 | rejects ("invalid designator kind") |

## Evidence

The rows "empty {}" and "designated {.x = 1}" of the lists grid on the
page (`tests/foundations/conversion-sequences/grids/lists.json`).

## Options

1. No sequence in either case. An array has no element for empty braces
   to give a bound, and a member designator names no array element
   (Clang).
2. Empty braces form an array of no elements, and a designated list
   is checked against the array (GCC, EDG in part).

## Provisional answer

Option 1. ICS-17 needs the elements to give the bound, and ICS-18 needs
the named members. Clang is the only compiler with one consistent
answer for both cases.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
