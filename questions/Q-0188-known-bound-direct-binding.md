# Q-0188: When two references bind directly to one array, is the reference to the known bound better?

**Status:** provisional
**Affects:** ICS-46, `tests/foundations/conversion-sequences/array-known-bound-direct.cpp`, `tests/foundations/conversion-sequences/list-string-literal-bound.cpp`
**Parent:** Q-0294, the model of braced lists for references, arrays, and conversion sequences

## Context

An array lvalue `int row[2]` is passed to `f(int (&)[2])` and
`f(int (&)[])`. Both references bind directly to `row`, with the
identity. No rule of the argument comparison tells the two apart.

| Compiler | `f(row)` | `f({row})` | an xvalue, with `&&` references | braced string literal |
|---|---|---|---|---|
| Clang 18, Clang 22 | tie | tie | tie | tie |
| GCC 14, GCC 16 | known bound | known bound | known bound | known bound |
| EDG 7.0 | known bound | known bound | known bound | known bound |

ICS-24 prefers the known bound for two list sequences that make
temporaries. Q-0153 covers pointers to arrays, where Clang prefers the
known bound and GCC and EDG find a tie.

## Evidence

The two tests in **Affects**, and probes with `const int` elements, which
give the same split.

## Options

1. The known bound is better for two direct bindings (GCC, EDG).
2. The two bindings tie (Clang).

## Provisional answer

Option 1. Two of three implementations agree, and the answer matches
ICS-24 for lists.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
