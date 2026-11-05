# Q-0102: Does a conditional with the literal 0 and nullptr have the type of nullptr?

**Status:** provisional
**Affects:** COND-3, `tests/expressions/conditional/zero-and-nullptr.cpp`

## Context

In `c ? 0 : nullptr`, one operand is the literal `0` and the other is `nullptr`. Clang and EDG accept the expression and give the result the type of `nullptr`. GCC rejects it and reports that the operand types differ.

## Options

1. The result has the type of `nullptr`.

2. The program is not valid.

## Provisional answer

Yes; the result has the type of `nullptr` (option 1). Clang and EDG agree.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page gives the answer. COND-P2 gives the general model of one result type, and no principle states the result for `nullptr` and the literal `0`. GCC 16 is the only newest family that differs.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). COND-P2 failed step 3, because no principle stated the result for `nullptr` and the literal `0`. Item PQ widened COND-P2: a null pointer constant has no pointer type of its own, so `nullptr` with `nullptr` or with the literal `0` gives the type of `nullptr`. COND-3 and its test `zero-and-nullptr.cpp` are already among the evidence of COND-P2. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses COND-P2.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. The last case of the conditional operator covers one operand of the type of `nullptr` and another that is a null pointer constant, and the result has the type of `nullptr`. The literal `0` is a null pointer constant, so GCC's rejection is a defect.
