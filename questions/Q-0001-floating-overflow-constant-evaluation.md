# Q-0001: Is floating overflow during constant evaluation undefined or implementation-defined?

**Status:** provisional
**Affects:** MUL-12, ADD-17, `tests/expressions/multiplicative/floating-overflow-constant.cpp`,
`tests/expressions/multiplicative/floating-overflow-divide-constant.cpp`,
`tests/expressions/additive/float-overflow-constant.cpp`,
`tests/expressions/additive/float-overflow-subtract-constant.cpp`

## Context

`1e308 * 10.0` overflows `double`. At run time on IEEE 754 targets the
result is positive infinity. During constant evaluation the
implementations disagree, and the newest versions disagree in the same
way as the older ones.

| Compiler | Overflow of `*`, `/`, `+`, or `-` in `float` or `double` | A NaN result, such as an infinity minus itself |
|---|---|---|
| Clang 22.1.8, 18.1.3 | accepts; the value is an infinity of the right sign | rejects: "floating point arithmetic produces a NaN" |
| GCC 16.2.1, 16.2.0, 14.2.0 | rejects: "is not a constant expression" | not reached, because the overflow is rejected |
| EDG 7.0 | rejects: "floating-point error" | not reached |

At run time every Clang and GCC configuration gives an infinity for
the overflow in `float`, `double`, and `long double`, at `-O0` and at
`-O2`. `-fsanitize=undefined` lets the overflow run. The matrix
targets report IEEE 754 types (MUL-20, Q-0004).

## Evidence

- `tests/RESULTS.md`, rows `floating-overflow-constant.cpp` and
  `float-overflow-constant.cpp`.
- `floating-overflow-divide-constant.cpp` (`1e308 / 1e-10`) and
  `float-overflow-subtract-constant.cpp` (`-3.0e38f - 3.0e38f`) give the
  same split for `/`, for `-`, and for `float`.
- Probes of `1.7e308 + 1.7e308`, `3e38f * 10.0f`, and an infinity minus
  itself in a `constexpr` initializer, on every compiler in the matrix.
- A run-time probe of the overflow in all three types, with and
  without `-fsanitize=undefined`, on every Clang and GCC configuration.

## Options

1. `undefined`. GCC and EDG conform. Clang's acceptance is an extension.
2. `impl-defined`. All three conform. Programs can't rely on either outcome
   without checking each implementation's documentation.
3. Split the rule as MUL-11 and CE-28 split floating division by zero.
   At run time, a type that follows IEEE 754 gives an infinity (an
   extension of MUL-20), and a type that does not has undefined
   behavior. Constant evaluation rejects the overflow in every type, by
   a rule of its own. Clang's acceptance is then a missing diagnostic.

## Provisional answer

Option 1. Two of three implementations reject, and a program that
relies on acceptance is not portable today. Option 3 matches every
run-time result and the treatment of division by zero (Q-0004). Option
3 needs a new constant-evaluation rule and a change of class for MUL-12
and ADD-17, so the rules follow option 1 until a decision adopts it.

## Opinions
