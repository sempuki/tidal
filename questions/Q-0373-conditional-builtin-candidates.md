# Q-0373: Which built-in candidates does `?:` have for two class operands that convert to a scoped enumeration or to `float`?

**Status:** provisional
**Affects:** COND-11, `tests/expressions/conditional/two-classes-scoped-enumeration.cpp`, `tests/expressions/conditional/two-classes-float.cpp`

## Context

COND-11 resolves `c ? a : b` over built-in candidates when `a` and `b`
are of two unrelated classes and neither converts to the other. The
compilers agree on most candidates and differ in two places.

| Operands convert to | Clang 18, 22, host | GCC 14, 16, host | EDG 7.0 |
|---|---|---|---|
| one scoped enumeration `Mode` | `Mode` | rejects ("operands to ?: have different types") | `Mode` |
| `float` and `float` | `float` | `float` | `double` |
| `float` and `int` | `float` | `float` | `double` |
| one unscoped enumeration | `int` | `int` | `int` |
| `char` and `char` | `int` | `int` | `int` |

## Evidence

The tests in Affects, `tests/expressions/conditional/two-classes-promoted-types.cpp`,
and probes that assert the result type of each pair.

## Options

1. The arithmetic candidates take each pair of promoted arithmetic types,
   with `float` among them, and each scoped enumeration type has a
   candidate `(T, T)` (Clang; GCC for `float`; EDG for the enumeration).
2. A scoped enumeration has no candidate (GCC).
3. The floating candidates start at `double` (EDG).

## Provisional answer

The candidates of option 1. Clang follows option 1 in both places, and
each other family departs in one place only. The promoted arithmetic
types of option 1 are the same set that the arithmetic operators use in
the table of OVL-P4.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
