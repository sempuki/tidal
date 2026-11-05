# Q-0194: Can an operand of a fold expression contain a binary operator without parentheses?

**Status:** settled
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

## Answer

No, it needs parentheses; EDG differs (option 1). Two of three implementations reject the fold, and option 1
keeps the grouping of a fold visible in the source. A reader need not
compare precedences to see which operator the fold applies.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page gives the answer. PRIM-P6 says what a fold stands for, and no principle says what an operand of a fold can be. EDG 7.0 is the only newest family that differs.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). PRIM-P6 failed step 3, because no principle said what an operand of a fold can be. Item PQ widened PRIM-P6: each operand binds more tightly than any binary operator, and an operand with a binary operator needs its own parentheses. PRIM-29 and its test `fold-operand-binary.cpp` are already among the evidence of PRIM-P6. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses PRIM-P6.

Used on 2026-11-04 by the source-reader gap review of the text phase 8 wrote on expressions, foundations, and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`), Part B. The review applied PRIM-P6 to the question's program, a fold whose operand holds a binary operator without parentheses (`fold-operand-binary.cpp`), without the rule text: each operand of a fold binds more tightly than any binary operator, so an operand with a binary operator needs parentheses. That is the provisional answer, and the model rules out the other options.

Settled on 2026-11-04 under decision 0054. The review supports the answer with no doubt, and PRIM-P6 gives the same answer under the test of the style guide (section 3). EDG 7.0 still differs, and the tests keep their overrides.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. The grammar makes each operand of a fold a cast expression. An operand with a binary operator outside parentheses does not fit, so the fold is not valid. EDG's acceptance is a defect.

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on expressions, foundations, and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`) agrees with option 1. Each operand of a fold is a cast expression in the grammar, so it cannot hold a binary operator without parentheses.
