# Q-0332: Does GCC 16 calling the left operand of `<<` twice under the shift sanitizer change SHF-8?

**Status:** compiler defect
**Affects:** SHF-8, SEQ-11, `tests/expressions/shift/operand-order-shift-sanitizer.cpp`, `tests/expressions/shift/operand-order-shift-sanitizer-constant.cpp`

## Context

SHF-8 evaluates the left operand of a shift before the right operand,
and SEQ-11 evaluates each operand at most once. GCC 16 with
`-fsanitize=shift` and `-Wall` evaluates a left operand of `<<` that
calls a `constexpr` function twice. The side effects of the call happen
twice, in the order left, right, left. The shift result is correct.

The repeat shows at run time and during constant evaluation. A clean-room
reader found it in Write report W2 and narrowed it with probes:

- The repeat needs both `-fsanitize=shift` (alone or inside
  `-fsanitize=undefined`) and `-Wall`.
- A function that is not `constexpr` is called once.
- The operator `>>` calls its left operand once.
- GCC 14, Clang 18, and Clang 22 call the function once in every
  configuration.

## Evidence

- `tests/expressions/shift/operand-order-shift-sanitizer.cpp`: the run
  counts three calls on GCC 16 and exits 1. Every other compiler counts
  two.
- `tests/expressions/shift/operand-order-shift-sanitizer-constant.cpp`:
  the static assertion that counts two calls fails on GCC 16 and holds
  on every other compiler.
- `reports/clean-room/W2/probe/p6.cpp` to `p10.cpp`.

## Options

1. Keep SHF-8 and SEQ-11, and treat the repeat as a GCC defect. A
   warning option changes the program's behavior, and no other
   configuration repeats the call.
2. Allow an implementation to evaluate a shift operand more than once
   under a sanitizer.

## Provisional answer

No; GCC defect (option 1). A warning option must not change what a
program does, and every other compiler and configuration evaluates the
operand once.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
