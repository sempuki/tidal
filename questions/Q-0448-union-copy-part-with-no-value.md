# Q-0448: After a trivial copy of a union in constant evaluation, does a part that had no value in the source have a value in the copy?

**Status:** provisional
**Affects:** CE-22, CE-P3, `tests/foundations/constant-evaluation/union-copy-part-with-no-value.cpp`, `tests/foundations/constant-evaluation/union-copy-part-with-no-value-read.cpp`
**Source:** X25-5 (`reports/clean-room/X25/report.md`, probes `a3.cpp` and `a3c.cpp`), rerun on the matrix in `reports/clean-room/roadmap-13/writer-probes/x25-constant.json` and `x25-rules.json`, and by the item F6 writer with `tools/probe_grid.py`.
**Waits for:** a review (doubt)

## Context

A union has an `int` array as its active member, and only the first
element has a value. During constant evaluation, the program copies
the union with its trivial copy constructor and then reads an element
of the copy.

| Form | Clang 18 and 22 | GCC 14 and 16 | EDG 7.0 |
|---|---|---|---|
| the copy, a move, and a copy assignment, then a read of the first element | accepts | accepts | accepts |
| the copy, then a read of the element with no value | rejects | rejects | accepts |
| the same with a class member `Point` whose `y` has no value, then a read of `y` | rejects | rejects | accepts |
| a copy assignment into a target whose element has a value, then a read of that element | rejects | rejects | rejects |
| the same copy of a struct that is not a union | rejects | accepts | rejects |

The last row is the case of Q-0115. The copy of a struct reads its
parts, and the copy of a union does not on any compiler.

## Evidence

The tests in **Affects** and the probes in **Source**.

## Options

1. The copy reads no part, and a part with no value in the source has
   no value in the copy. A later read of the part is rejected (Clang,
   GCC).
2. The copy gives the part a value, and a later read is valid (EDG for
   the copy constructor; no compiler for the copy assignment).

## Provisional answer

The part has no value in the copy, and a read of it is rejected
(option 1). CE-22 says so. Clang and GCC agree, and EDG alone differs.
EDG also rejects the read after a copy assignment, so EDG's result
depends on the form of the copy.

Kept open on 2026-11-11 under decision 0054 (decision 0064, item RV9). The gap review of the phase 16 text supports the answer with a doubt (RV9-10), and EDG 7.0 alone differs.

Rule text on 2026-11-11 (decision 0064, item RV9): CE-22 adds the limit that RV9-10 reads, for a union whose active member is `mutable`. Q-0449 holds that case and its splits. The answer of this question is unchanged.

Kept open on 2026-11-11 under decision 0054 (decision 0066, item DQ). The second review supports the answer with a narrower doubt (DQ-29), and EDG 7.0 alone differs. The doubt touches only `union-copy-part-with-no-value.cpp`, and the read test stands under either reading.

## Opinions

### Gap review, 2026-11-11 (phase 16 text)

The source-reader gap review of the phase 16 text (`reports/review/2026-11-11-gap-review-phase-16.md`) reads the question: supports with a doubt (RV9-10). For constant evaluation, the trivial copy or move of a union copies its active member, if it has one, and that copy is trivial. Read as a copy of the active member's representation, the copy reads no part, and a part with no value in the source has none in the target (option 1), as Clang and GCC say. The doubt is that the reference does not choose in words between that reading and a copy of each part. Under the second reading the copy itself would read the element with no value and would not be constant, and every compiler reads against that. The review adds a condition that CE-22 does not state: the implicit copy or move of a union whose active member is `mutable` is not constant, unless the union's lifetime began in the evaluation.

### Read of the RV9 repair diff, 2026-11-11

The source-reader read of the RV9 repair diff (`reports/review/2026-11-11-read-of-rv9-repair-diff.md`) reads the gap review's entry on the question: agrees as a record. The entry states RV9-10 correctly, with the limit on a `mutable` active member, and the doubt of RV9-10 on the form of the copy stays. The read's doubt on the new CE-22 sentence (RV10-R-4) concerns the case of Q-0449. The read gives no new reading of this question's answer.

### Second review of the doubt-held questions, 2026-11-11

The source-reader second review of the questions held only by a doubt (`reports/review/2026-11-11-second-review-doubt-held-questions.md`) reads the question: supports with a doubt that remains (DQ-29). For constant evaluation, a trivial copy or move of a union copies its active member, and that copy is trivial. The reference does not say whether a trivial copy of the active member copies its representation or each part. The doubt now touches only the acceptance test `union-copy-part-with-no-value.cpp`. A copy part by part would read the element or member with no value, and the copy itself would not be constant, which every compiler reads against. The read test `union-copy-part-with-no-value-read.cpp` is rejected under either reading, at the copy or at the read, so it stands as written.
