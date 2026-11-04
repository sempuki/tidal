# Q-0030: May a pointer be compared with `<` or `<=>` to the literal 0?

**Status:** settled
**Affects:** CMP-9, CMP-13, `tests/expressions/comparison/pointer-relational-zero.cpp`, `tests/expressions/comparison/three-way-pointer-zero.cpp`, `tests/expressions/comparison/pointer-relational-nullptr.cpp`

## Context

`p < 0` and `p <=> 0`, where `p` is a pointer, are rejected by Clang 22
and 18 and GCC 16 and 14. EDG 7.0 accepts both. Equality comparison
with `0` (`p == 0`) is accepted everywhere.

EDG's acceptance follows the null pointer constant:

| Right operand of `p <` | Clang, GCC | EDG 7.0 |
|---|---|---|
| `0` | rejects | accepts |
| `0L` | rejects | accepts |
| `nullptr` | rejects | accepts |
| `1` | rejects | rejects |
| a `constexpr int` variable whose value is zero | rejects | rejects |

During constant evaluation EDG rejects `&x > 0` and `&x > nullptr`, so
EDG accepts the comparison but gives it no constant value.

## Evidence

The tests in Affects, and probes of the operands in the table on every
compiler in the matrix. EDG 7.0 is the only EDG version in the matrix.

## Options

1. Rejected. Ordered comparisons need two pointers, and EDG's acceptance
   is a possible defect or extension.
2. Valid, with `0` converted to a null pointer as for equality.

## Answer

No; rejected (option 1). Clang and GCC, in both versions, reject both comparisons.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) supports this answer with no doubt, and CMP-P2 and CMP-P5 give the same answer: a pointer compares with a literal `0` for equality only, because a null pointer has no order, and `<=>` rejects a pointer against `0`. Tidal records EDG 7.0's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for EDG 7.0.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. A relational operator converts its operands to a composite pointer type only when both are pointers. A pointer and an integer zero take neither the arithmetic path nor the pointer path, so the operands have different types and the comparison is not valid. `nullptr` is neither arithmetic nor a pointer, so a comparison with it is not valid either. `<=>` needs an object pointer or an array on both sides. EDG's acceptance is a defect.
