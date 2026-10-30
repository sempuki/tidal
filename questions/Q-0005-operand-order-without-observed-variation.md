# Q-0005: Can operand order be `unspecified` when no compiler varies it?

**Status:** provisional
**Affects:** MUL-13, `docs/pipeline.md` step 5

## Context

Clang and GCC evaluate the left operand of the built-in `*`, `/`, and `%`
first at `-O0` and `-O2`. The pipeline normally requires variation as
evidence for `unspecified`. For an overloaded `*`, GCC evaluates the right
operand first and Clang evaluates the left operand first.

## Evidence

`tests/expressions/multiplicative/operand-order.cpp` and
`tests/expressions/multiplicative/operand-order-overloaded.cpp` (both accept
either order).

## Options

1. `unspecified`, on the evidence of the overloaded form. Programs must not rely on order.
2. `required` left to right, matching all observed behavior.

## Provisional answer

Option 1. Choosing option 2 would make Tidal stricter than the
implementations claim to be, which would break compatibility the first
time an implementation reorders.

## Opinions
