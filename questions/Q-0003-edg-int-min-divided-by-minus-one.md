# Q-0003: Does EDG accepting `INT_MIN / -1` during constant evaluation change MUL-10?

**Status:** compiler defect
**Affects:** MUL-10, `tests/expressions/multiplicative/signed-overflow-divide.cpp`, `tests/expressions/multiplicative/signed-overflow-remainder.cpp`, `tests/expressions/multiplicative/signed-overflow-divide-long-long.cpp`

## Context

The most negative `int` divided by `-1` gives a quotient one larger than
the largest `int`. Clang 22 and 18 and GCC 16 and 14 reject
`INT_MIN / -1` and `INT_MIN % -1` during constant evaluation. EDG 7.0
accepts both. The same holds for `long long`.

EDG detects the same overflow in other forms. EDG rejects `-INT_MIN`,
`INT_MIN * -1`, and `INT_MAX * 2`. EDG's value for `INT_MIN / -1`
passes a `static_assert` that the value is greater than zero, and fails
one that the value equals `INT_MAX`, so the value lies outside the range
of `int`. EDG gives `0` for `INT_MIN % -1`.

At run time, `INT_MIN / -1` and `INT_MIN % -1` stop the program with a
hardware exception on every Clang and GCC configuration, at `-O0` and
at `-O2`. `-fsanitize=undefined` reports both operators as "division of
-2147483648 by -1 cannot be represented".

## Evidence

- `tests/RESULTS.md`, rows `signed-overflow-divide.cpp` and
  `signed-overflow-remainder.cpp`.
- `signed-overflow-divide-long-long.cpp` and
  `signed-overflow-remainder-sanitizer.cpp`.
- Probes of `-INT_MIN`, `INT_MIN * -1`, and the value of
  `INT_MIN / -1` on EDG 7.0, the only EDG version in the matrix.

## Options

1. Keep MUL-10 as `undefined`, and report EDG's behavior to the EDG
   project as a possible defect.
2. Treat `INT_MIN % -1` separately, since its mathematical result (zero) is
   representable.

## Provisional answer

No; EDG's result is likely a defect, for both operators (option 1). Clang and GCC treat `%` the same as `/`
during constant evaluation, at run time, and in the sanitizer. EDG's
quotient lies outside the range of `int`, which no defined result
allows.

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 reject `INT_MIN / -1` and `INT_MIN % -1` for `int` and `long long`, and EDG 7.0 accepts them, so two families disagree. Probes confirm that EDG 7.0 still rejects `-INT_MIN` and `INT_MIN * -1` and gives 0 for `INT_MIN % -1`. CE-P3 supports the provisional answer, because constant evaluation rejects any operation that is undefined (CE-9, MUL-10). The report of EDG's behavior to the EDG project waits on the maintainer. The front end's interpreter rejects both operators with MUL-10.

## Opinions
