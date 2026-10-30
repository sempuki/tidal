# Q-0005: Can operand order be `unspecified` when no compiler varies it?

**Status:** provisional
**Affects:** MUL-13, `docs/pipeline.md` step 5

## Context

Clang and GCC evaluate the left operand of `*`, `/`, and `%` first at
`-O0` and `-O2`. The pipeline normally requires variation as evidence for
`unspecified`. GCC's sanitized source applies its strict-order option to
subscripts and shifts, and not to these operators.

## Evidence

`tests/expressions/multiplicative/operand-order.cpp` (accepts either order);
`flag_strong_eval_order` uses in `gcc/cp/typeck.cc` at GCC 16.2.0.

## Options

1. `unspecified`, on source evidence. Programs must not rely on order.
2. `required` left to right, matching all observed behavior.

## Provisional answer

Option 1. Choosing option 2 would make Tidal stricter than the
implementations claim to be, which would break compatibility the first
time an implementation reorders.

## Opinions
