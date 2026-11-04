# Q-0029: Is a chained comparison such as `3 > 2 > 1` valid?

**Status:** provisional
**Affects:** CMP-1, CMP-14, `tests/expressions/comparison/chained-comparison.cpp`, `tests/expressions/comparison/chained-comparison-warns.cpp`, `tests/expressions/comparison/chained-less-equal-warns.cpp`, `tests/expressions/comparison/chained-class-bool-warns.cpp`, `tests/expressions/comparison/chained-fold-not-chain.cpp`, `tests/expressions/comparison/chained-requires-not-dependent.cpp`, `tests/expressions/comparison/chained-in-concept-no-warning.cpp`, `tests/expressions/comparison/chained-equality-gcc-warns.cpp`, `tests/expressions/comparison/chained-relational-equality-gcc-warns.cpp`

## Context

`3 > 2 > 1` groups as `(3 > 2) > 1`, which compares a `bool` with `1`.

| Compiler | `3 > 2 > 1` with no warning flags | `a < b < c` with `-Werror -Wparentheses` |
|---|---|---|
| Clang 22 (host and container) | rejects | rejects |
| Clang 18 | accepts | accepts, with no warning |
| GCC 16 and 14 | accepts | rejects |
| EDG 7.0 | accepts | not run; EDG has no such flag |

Clang 22's rejection comes from a `-Wparentheses` warning whose default
severity is an error. With `-Wno-error=parentheses`, Clang 22 gives the
warning and accepts the program. With `-Wno-parentheses`, Clang 22 gives
nothing. Clang 22 accepts the same expression with parentheses,
`(3 > 2) > 1`. GCC also warns about `a == b == c` and `a < b == c`, and
Clang 22 does not.

The results differ by form and context:

| Form | Clang 22 | Clang 18 | GCC 16 and 14 |
|---|---|---|---|
| `a < b < c` where `<` is an operator function that returns `bool`, `-Werror -Wparentheses` | accepts, no warning | accepts, no warning | rejects |
| A fold `(... < p)`, no flags | rejects | accepts | accepts, no warning under `-Wall` |
| A chain in a requires expression that depends on no template parameter, no flags | rejects | accepts | accepts, no warning under `-Wall` |
| A chain in a dependent requirement or a template never instantiated, `-Werror -Wparentheses` | accepts | accepts | accepts |
| `a == b == c` or `a < b == c`, `-Werror -Wparentheses` | accepts | accepts | rejects ("suggest parentheses around comparison in operand of `==`") |

Each program is valid under the provisional answer. CMP-14 does not
count a fold as a chained comparison, because each nested part of a
fold acts as if it were in parentheses (PRIM-27).

## Evidence

The tests in Affects, and probes of the flags and forms above on every
compiler in the matrix. The newest Clang is the one that rejects, so no
newer version settles the split.

## Options

1. Valid. Rejecting a chained comparison by default is an implementation's
   diagnostic choice, and a warning is recommended practice.
2. An implementation may reject it.

## Provisional answer

Valid; a warning is recommended (option 1). The grouping and result are well defined, and four of five
compiler versions accept the expression. Clang 22 treats the rejection
as a warning that a flag turns back into a warning, which matches
option 1 with a strict default.

## Opinions
