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

Reopened as provisional on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). The gap review of the rules that the phase 7 front-end and Propose repairs rewrote (`reports/review/2026-11-04-gap-review-phase-7-repairs.md`) supports this answer with no doubt. OVL-P4 fails the test. OVL-P4 is a principle of the overloading page, and no principle or rule of the conditional page cites OVL-P4. The `?:` row of the OVL-P4 table also sends the reader to COND-11 instead of giving the candidates. No other principle on the page gives the answer. GCC 16 and EDG 7.0 each differ in one form, and the tests keep their overrides.

On 2026-11-04, item PQ widened COND-P2 to state the case and to cite OVL-P4. COND-P2 now lists the built-in candidates of `?:` for two class operands: every pair of promoted arithmetic types, `float` among them, and `(T, T)` for each pointer, pointer-to-member, and scoped enumeration type. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses COND-P2.

## Opinions

### Gap review, 2026-11-04

The source-reader gap review of the rules that the phase 7 front-end and Propose repairs rewrote (`reports/review/2026-11-04-gap-review-phase-7-repairs.md`) agrees with option 1. The arithmetic candidates take every floating-point type, the single-precision type included, so two operands that convert to `float` give `float`. Each scoped enumeration type has its own candidate. GCC and EDG each conflict with the reference in one place.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
