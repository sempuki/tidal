# Q-0005: Can operand order be `unspecified` when no compiler varies it?

**Status:** settled
**Affects:** MUL-13, ADD-13, `documentation/pipeline.md` step 5

## Context

Clang and GCC evaluate the left operand of the built-in `*`, `/`, `%`,
`+`, `<`, and `==` first, at `-O0` and at `-O2`. The pipeline normally
requires variation as evidence for `unspecified`. For an overloaded
operator, GCC evaluates the right operand first and Clang evaluates the
left operand first.

| Form | Clang 18, 22, host, `-O2` | GCC 14, 16, host, `-O2` |
|---|---|---|
| built-in `*`, `/`, `%`, `<`, `==` on `int`, `double`, `long long` | left first | left first |
| built-in `*` on class operands that convert to `int` | left first | left first |
| built-in `*` on members of class prvalues, on references, and on nested calls | left first | left first |
| overloaded `*`, `/`, `+`, `<`, `==` as non-member functions | left first | right first |

The two operands of a built-in operator are unsequenced (SEQ-8). Clang
and GCC warn about a modification of a scalar in one operand and an
access in the other (MUL-14).

## Evidence

`tests/expressions/multiplicative/operand-order.cpp`,
`tests/expressions/multiplicative/operand-order-overloaded.cpp`,
`tests/expressions/additive/operand-order.cpp`, and
`tests/expressions/additive/operand-order-overloaded.cpp` (all accept
either order). A probe of the forms in the table on every Clang and GCC
configuration.

## Options

1. `unspecified`, on the evidence of the overloaded form. Programs must
   not rely on order.
2. `required` left to right, matching all observed behavior.

## Answer

Yes, on source evidence (option 1). Choosing option 2 would make Tidal stricter than the
implementations claim to be, which would break compatibility the first
time an implementation reorders. The built-in forms show no variation
in the matrix, and SEQ-8 leaves their two operands unsequenced.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) supports this answer with no doubt, and MUL-P4 and ADD-P4 give the same answer: the two operands are not ordered with respect to each other. The review also closes the built-in forms that the answer left open, because the two operands of a built-in operator are unsequenced, which allows any order. No newest family differs from the answer.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. The two operands of a built-in operator of this kind are unsequenced, which is stronger than an unspecified order, because the two evaluations can also interleave. An overloaded operator written as an operator is a function call, and the order of a call's arguments is not fixed for these operators. The review finds that the lack of variation in the built-in forms does not change the answer.
