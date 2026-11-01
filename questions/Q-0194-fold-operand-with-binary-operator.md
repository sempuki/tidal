# Q-0194: Can an operand of a fold expression contain a binary operator without parentheses?

**Status:** provisional
**Affects:** PRIM-29, `tests/expressions/primary/fold-operand-binary.cpp`

## Context

Take `(2 * V + ...)`, where `V` is a pack. The operand `2 * V` holds a
binary operator and has no parentheses. Clang 18, Clang 22, GCC 14, and
GCC 16 reject the fold. Clang says that the expression is not permitted
as an operand of a fold expression. GCC names the binary expression in
the operand. EDG 7.0 accepts the fold. Every compiler accepts
`((2 * V) + ...)` and `(-V + ...)`.

## Evidence

- `tests/expressions/primary/fold-operand-binary.cpp`
- `tests/expressions/primary/fold-expansion.cpp`

## Options

1. An operand of a fold is a cast expression. A binary operator in an
   operand needs parentheses. EDG differs.
2. An operand can hold a binary operator of higher precedence than the
   fold operator. Clang and GCC differ.

## Provisional answer

Option 1. Two of three implementations reject the fold, and option 1
keeps the grouping of a fold visible in the source. A reader need not
compare precedences to see which operator the fold applies.

## Opinions
