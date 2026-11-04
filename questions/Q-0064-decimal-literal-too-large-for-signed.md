# Q-0064: Is a decimal literal that fits only an unsigned type valid?

**Status:** settled
**Affects:** PRIM-2, `tests/expressions/primary/int-too-large-signed-decimal.cpp`

## Context

The literal `9223372036854775808` has no suffix and does not fit in
`long long`. It fits in `unsigned long long`. Clang and GCC reject it
under `-pedantic-errors`. Without that option they warn and give the
literal an unsigned type. EDG 7.0 accepts the literal in strict mode.

## Options

1. The program is not valid.
2. The literal has an unsigned type.

## Answer

No; the program is not valid (option 1). Clang and GCC treat the unsigned type as an extension.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) supports this answer with no doubt, and PRIM-P1 gives the same answer: the base and the suffix fix a list of types, no context changes the type, and a literal that no type in its list can hold is rejected. EDG 7.0 still differs. Tidal records EDG 7.0's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for EDG 7.0.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. A decimal literal without a suffix has a list of signed types only. A value that none of them holds may take a signed extended integer type, and otherwise the program is not valid. An unsigned type is never allowed. EDG's acceptance in strict mode is a defect unless EDG has a signed extended type wide enough.
