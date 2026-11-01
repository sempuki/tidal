# Q-0004: Should floating division by zero be implementation-defined on IEEE 754 targets?

**Status:** provisional
**Affects:** MUL-11, MUL-20, CE-28, CMP-5, CMP-17, CONV-3,
`tests/expressions/multiplicative/floating-divide-by-zero-infinity.cpp`,
`tests/expressions/multiplicative/floating-zero-by-zero-nan.cpp`,
`tests/expressions/multiplicative/floating-divide-by-zero-sanitizer.cpp`,
`tests/expressions/multiplicative/ieee-types.cpp`,
`tests/expressions/comparison/nan.cpp`,
`tests/expressions/comparison/three-way-floating.cpp`,
`tests/expressions/conversions/nan-to-bool.cpp`.

## Context

A floating-point `/` with a zero right operand has no mathematical
result. A type that follows IEEE 754 has infinities and NaN values, and
gives one of them.

| Case | Clang 18, 22, host | GCC 14, 16, host | EDG 7.0 |
|---|---|---|---|
| run time, `-O0` and `-O2` | an infinity, or a NaN for `0.0 / 0.0` | the same | no code generation |
| `-fsanitize=undefined` | runs, no report | runs, no report | not run |
| `-fsanitize=float-divide-by-zero` | stops | stops | not run |
| constant evaluation | rejects, "division by zero" | rejects, "not a constant expression" | accepts (Q-0002) |

Every Clang and GCC configuration in the matrix reports IEEE 754 types
for `float`, `double`, and `long double`, on x86-64 and with `-m32`. The
programs that test NaN comparisons need a NaN, and Tidal has no other
way to make one.

## Evidence

The tests in Affects. Probes with `1.0 / 0.0`, `-1.0 / 0.0`,
`1.0 / -0.0`, and `0.0 / 0.0` in all three types gave the IEEE 754
results on every configuration, including `-O2`. A test of
`__STDC_IEC_559__` and of the library's report of IEEE 754 types passed
on every Clang and GCC configuration.

## Options

1. `undefined` everywhere. The NaN tests have no valid program, and
   every configuration's result is one that undefined behavior allows.
2. `impl-defined`. Each implementation chooses whether each floating
   type follows IEEE 754 (MUL-20). In a type that follows it, the
   division gives the IEEE 754 result. In a type that does not, the
   behavior is undefined. Constant evaluation rejects the division in
   every type by a rule of its own (CE-28).
3. `undefined` only in constant evaluation, and an IEEE 754 result at
   run time on every implementation. Tidal would then require IEEE 754
   of every implementation.

## Provisional answer

Option 2. The choice is a property of the implementation, which the
matrix implementations make and report the same way at every
optimization level (decision 0047). The general sanitizer does not treat
the division as undefined. Constant evaluation rejects the division on
Clang and GCC, so CE-28 states that limit apart from the run-time
result.

MUL-12 keeps floating overflow `undefined` (Q-0001). The same argument
can apply to an overflow at run time in a type that follows IEEE 754,
and a later change can extend MUL-20 to it.

## Opinions

- Spec-only Predict reader, 2026-10-31 (`reports/spec-only/R-foundations/`,
  finding C1): CONV-3 says a NaN converts to `true`, but under option 1 a
  program has no way to make a NaN without undefined behavior, because
  the language offers no NaN literal and the library is out of scope.
  Option 2 would give CONV-3 a valid example.
