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

No, it needs parentheses; EDG differs (option 1). Two of three implementations reject the fold, and option 1
keeps the grouping of a fold visible in the source. A reader need not
compare precedences to see which operator the fold applies.

Kept open on 2026-11-03 under decision 0054, because no principle on the page gives the answer. PRIM-P6 says what a fold stands for, and no principle says what an operand of a fold can be. EDG 7.0 is the only newest family that differs.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). PRIM-P6 failed step 3, because no principle said what an operand of a fold can be. Item PQ widened PRIM-P6: each operand binds more tightly than any binary operator, and an operand with a binary operator needs its own parentheses. PRIM-29 and its test `fold-operand-binary.cpp` are already among the evidence of PRIM-P6. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses PRIM-P6.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. The grammar makes each operand of a fold a cast expression. An operand with a binary operator outside parentheses does not fit, so the fold is not valid. EDG's acceptance is a defect.
