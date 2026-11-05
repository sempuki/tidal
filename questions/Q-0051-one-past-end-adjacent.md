# Q-0051: Is comparing a one-past-the-end pointer with a pointer to the next object rejected during constant evaluation?

**Status:** settled
**Affects:** CMP-20, `tests/expressions/comparison/one-past-end-adjacent-constant.cpp`

## Context

Whether a pointer one past the end of an object equals a pointer to an
object that follows it in memory depends on layout, so the result is
unspecified. During constant evaluation, Clang 22 and 18 and GCC 16 and
14 reject the comparison. EDG 7.0 accepts it.

Inside one array of arrays, every implementation accepts the same kind
of comparison during constant evaluation. The pointer one past the end
of one row equals the pointer to the first element of the next row
(CMP-26). The rejection applies only when the two pointers lie in
different complete objects, whose layout the implementation chooses.

## Evidence

The test in Affects, `tests/expressions/comparison/row-end-equals-next-row.cpp`,
and a new probe on every compiler in the matrix. EDG 7.0 is the only EDG
version in the matrix.

## Options

1. Rejected during constant evaluation; unspecified at run time.
2. Accepted during constant evaluation, with an unspecified value.

## Answer

Rejected during constant evaluation, and unspecified at run time (option 1). Clang and GCC agree, and option 1 matches Q-0033.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page gives the answer. CMP-P3 makes the result unspecified and lists CMP-20 as an exception, and no principle on the page states the rejection during constant evaluation. EDG 7.0 is the only newest family that differs.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). CMP-P3 failed, because CMP-P3 named CMP-20 in its `:exceptions:` line. CE-P3 gives the model, and the comparison page did not cite CE-P3. Item PQ widened CMP-P3 to cite CE-P3 and to state the case: during constant evaluation a comparison that depends on the placement of complete objects is rejected, and inside one complete object it is accepted. CMP-8 and CMP-20 move from the `:exceptions:` line to the `:rules:` line of CMP-P3. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses CMP-P3.

Used on 2026-11-04 by the source-reader gap review of the text phase 8 wrote on expressions, foundations, and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`), Part B. The review applied CMP-P3 to the question's program, the equality of a pointer past the end of one complete object with a pointer to another complete object, in constant evaluation (`one-past-end-adjacent-constant.cpp`), without the rule text: the result depends on where the objects are placed, so constant evaluation rejects the comparison, which is the provisional answer.

Kept open on 2026-11-04 under decision 0055, because CMP-P3 changed in this commit. The same review found the last paragraph of CMP-P3 too wide for relational comparisons that involve a base class subobject, and CMP-P3 now lists them (Q-0387). The question waits for the next use of CMP-P3. EDG 7.0 is the one newest family that differs.

Used on 2026-11-04 by the principle-use batch PU1 (`reports/clean-room/PU1/`), under decision 0056. A clean-room reader applied CMP-P3, without the rule text, to the programs of `row-end-equals-next-row.cpp` and `one-past-end-adjacent-constant.cpp`. Each answer names CMP-P3 and gives the outcome that the tests expect, so the use supports the answer.

Settled on 2026-11-04 under decision 0054. The use in batch PU1 supports the answer, the gap review of 2026-11-04 supports it with no doubt, and CMP-P3 gives the same answer by the test of the style guide (section 3). EDG 7.0 still differs, and the test keeps its override. Finding PU1-8 asks for an example of the row case in CMP-P3. The reader's answer for `row-end-equals-next-row.cpp` follows the principle as written.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. The result is unspecified when one pointer is one past the end of one complete object and the other points to a different complete object. An equality comparison with an unspecified result is not allowed during constant evaluation, so EDG's acceptance is a defect.

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on expressions, foundations, and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`) agrees with option 1. For equality, only a pointer past the end of one complete object against another complete object has no fixed result, and constant evaluation rejects a comparison whose result is unspecified. Inside one complete object, equality compares addresses and has a fixed result.
