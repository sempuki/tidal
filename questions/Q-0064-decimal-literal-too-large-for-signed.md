# Q-0064: Is a decimal literal that fits only an unsigned type valid?

**Status:** provisional
**Affects:** PRIM-2, `tests/expressions/primary/int-too-large-signed-decimal.cpp`

## Context

The literal `9223372036854775808` has no suffix and does not fit in
`long long`. It fits in `unsigned long long`. Clang and GCC reject it
under `-pedantic-errors`. Without that option they warn and give the
literal an unsigned type. EDG 7.0 accepts the literal in strict mode.

## Options

1. The program is not valid.
2. The literal has an unsigned type.

## Provisional answer

No; the program is not valid (option 1). Clang and GCC treat the unsigned type as an extension.

Reopened as provisional on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) supports this answer with no doubt. PRIM-P1 fails step 3 of the test. The model says that the base and the suffix fix a list of types and that a literal no type in the list can hold is rejected. The model does not say which types the list of a decimal literal with no suffix holds, so the answer turns on the table of PRIM-2. No other principle on the page gives the answer. EDG 7.0 differs, and the tests keep their overrides for EDG 7.0.

On 2026-11-04, item PQ widened PRIM-P1 to state the case. PRIM-P1 now gives the list of types for each base and suffix in a table. A decimal literal with no `u` has only signed types in its list, so a decimal value that only `unsigned long long` can hold is rejected. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses PRIM-P1.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. A decimal literal without a suffix has a list of signed types only. A value that none of them holds may take a signed extended integer type, and otherwise the program is not valid. An unsigned type is never allowed. EDG's acceptance in strict mode is a defect unless EDG has a signed extended type wide enough.
