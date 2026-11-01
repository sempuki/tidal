# Q-0030: May a pointer be compared with `<` or `<=>` to the literal 0?

**Status:** provisional
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

## Provisional answer

No; rejected (option 1). Clang and GCC, in both versions, reject both comparisons.

## Opinions
