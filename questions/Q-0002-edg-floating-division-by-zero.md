# Q-0002: Does EDG accepting floating division by zero during constant evaluation change CE-28?

**Status:** compiler defect
**Affects:** CE-28, MUL-11, `tests/expressions/multiplicative/floating-divide-by-zero.cpp`, `tests/expressions/multiplicative/floating-zero-by-zero.cpp`

## Context

CE-28 rejects a floating-point `/` with a zero right operand during
constant evaluation, also in a type that follows IEEE 754. Clang 22 and
18 and GCC 16 and 14 reject `1.0 / 0.0` and `0.0 / 0.0` during constant
evaluation. EDG 7.0 accepts both, and gives an infinity and a NaN: a
`static_assert` that the first value exceeds `1e308` and that the
second value differs from itself passes. EDG rejects floating overflow
(Q-0001), so EDG treats some floating-point results as errors and not
others.

## Evidence

`tests/RESULTS.md`, rows `floating-divide-by-zero.cpp` and
`floating-zero-by-zero.cpp`. A probe of the two values in `constexpr`
initializers on every compiler in the matrix. EDG 7.0 is the only EDG
version in the matrix. EDG 7.0 also accepts the division as the
initializer of a `constexpr` variable, as a `float` division, and in a
template argument.

## Options

1. Keep CE-28, and report EDG's behavior to the EDG project as a
   possible defect.
2. Allow constant evaluation to give the IEEE 754 result, with EDG's
   acceptance as one permitted outcome.

## Provisional answer

No; likely EDG defect (option 1). Two of three implementations reject, and EDG's own handling
is inconsistent with its rejection of overflow.

The question stays open under decision 0051 (2026-11-03). Clang 22 and GCC 16 reject both divisions, and EDG 7.0 accepts them, so two families disagree. A probe confirms that EDG 7.0 still gives an infinity and a NaN. CE-P3 supports the provisional answer. An evaluation must not depend on the machine, and the result of a floating division by zero depends on the machine (CE-28). The report of EDG's behavior to the EDG project waits on the maintainer. The front end's interpreter rejects both divisions with CE-28.

Kept on 2026-11-03. The source-reader gap review of question cluster 4 agrees with option 1.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 4 (`reports/review/2026-11-03-gap-review-question-cluster-4.md`) reads the reference as supporting option 1. A zero divisor is undefined for every arithmetic type, floating types included, with no exception for a type that follows IEEE 754. Undefined behavior keeps the expression from being constant.
