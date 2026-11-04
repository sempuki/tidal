# Q-0103: Can two values of the type of nullptr be compared with a relational operator?

**Status:** settled
**Affects:** CMP-9, `tests/expressions/comparison/relational-nullptr-values.cpp`

## Context

`a < b` where both operands have the type of `nullptr`. Clang 22 and 18
and GCC 16 and 14 reject the comparison. EDG 7.0 accepts `<`, `<=`, and
`>=`, and during constant evaluation gives `false`, `true`, and `true`.
EDG rejects `<=>` on the same operands
(`three-way-nullptr-values.cpp`), so EDG orders the two values with
one operator and not with the other.

## Evidence

The test in Affects, `tests/expressions/comparison/three-way-nullptr-values.cpp`,
and a probe of the three operators in a `static_assert` on every
compiler in the matrix. EDG 7.0 is the only EDG version in the matrix.

## Options

1. The comparison is not valid, as for two pointers to members.
2. The comparison is valid.

## Answer

No, as for two pointers to members (option 1). Clang and GCC agree, and only `==` and `!=` apply to the type
of `nullptr`. EDG's rejection of `<=>` agrees with option 1.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) supports this answer with no doubt, and CMP-P2 gives the same answer: a null pointer has no order, so it compares for equality only. EDG 7.0 still differs. Tidal records EDG 7.0's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for EDG 7.0.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. The relational operators take arithmetic, enumeration, or pointer operands after the usual conversions, and the type of `nullptr` is none of these. Only the equality operators list that type. A diagnostic is required, so EDG's acceptance is a defect.
