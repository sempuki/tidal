# Q-0356: Does GCC 14 evaluating the right operand of `.*` first, when the left operand is a comma expression, change SEQ-2?

**Status:** compiler defect
**Affects:** SEQ-2, `tests/programs/W2/order.cpp`

## Context

SEQ-2 evaluates the left operand of `a.*b` before the right operand.
In the W2 Write program each operand is a comma expression that first
records its turn and then gives an object or a member pointer:
`(L.mark(1), p).*(L.mark(2), mp)`.

| Compiler | Run time | Constant evaluation |
|---|---|---|
| Clang 18, Clang 22 | left first | left first |
| GCC 14 | right first | right first |
| GCC 16 | left first | left first |
| EDG 7.0 | no code generation | left first |

GCC 14 takes the left operand first when the left operand is a call
that returns a reference, and for `->*` with a comma expression on the
left. `tests/expressions/sequencing/member-pointer-object-first.cpp`
passes on GCC 14.

## Evidence

`tests/programs/W2/order.cpp`, the W2 Write program, with an override
for GCC 14. Probes by the clean-room writer on the matrix containers
gave the table above.

## Options

1. The order holds for every form of the operands (SEQ-2). GCC 14's
   order is a defect that GCC 16 fixes.
2. The order of `.*` is unspecified when the left operand is a comma
   expression.

## Provisional answer

No; the left operand comes first, and GCC 14 is wrong (option 1). The
newest version of each family agrees, and SEQ-2 gives the order with no
condition on the form of the operands.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
