# Q-0051: Is comparing a one-past-the-end pointer with a pointer to the next object rejected during constant evaluation?

**Status:** provisional
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

## Provisional answer

Rejected during constant evaluation, and unspecified at run time (option 1). Clang and GCC agree, and option 1 matches Q-0033.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page gives the answer. CMP-P3 makes the result unspecified and lists CMP-20 as an exception, and no principle on the page states the rejection during constant evaluation. EDG 7.0 is the only newest family that differs.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). CMP-P3 failed, because CMP-P3 named CMP-20 in its `:exceptions:` line. CE-P3 gives the model, and the comparison page did not cite CE-P3. Item PQ widened CMP-P3 to cite CE-P3 and to state the case: during constant evaluation a comparison that depends on the placement of complete objects is rejected, and inside one complete object it is accepted. CMP-8 and CMP-20 move from the `:exceptions:` line to the `:rules:` line of CMP-P3. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses CMP-P3.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. The result is unspecified when one pointer is one past the end of one complete object and the other points to a different complete object. An equality comparison with an unspecified result is not allowed during constant evaluation, so EDG's acceptance is a defect.
