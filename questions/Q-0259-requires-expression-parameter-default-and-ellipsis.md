# Q-0259: Can a parameter of a requires expression have a default argument, or the list end in an ellipsis?

**Status:** settled
**Affects:** CPT-29, `tests/templates/concepts/requires-parameter-default.cpp`, `tests/templates/concepts/requires-parameter-ellipsis.cpp`

## Context

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects both |
| GCC 14, 16, host | accepts both |
| EDG 7.0 | rejects both |

## Evidence

The two tests above. Clang's diagnostic list has an error for each case (clang-diag-sema, clang-diag-parse).

## Options

1. Not valid (Clang, EDG). The parameters are only names with types,
   and no call ever gives them arguments.
2. Valid (GCC).

## Answer

No; Clang and EDG agree, GCC differs (option 1). Two of three implementations agree, and a default argument or an ellipsis has no use where nothing is called.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) supports this answer with no doubt, and CPT-P3 gives the same answer: the parameters of a requires expression are only names with types, and no objects or calls exist for them. GCC 16 still differs. Tidal records GCC 16's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for GCC 16.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) agrees with option 1. A default argument on a parameter of a requires expression, and a parameter list there that ends with an ellipsis, are both forbidden, and neither ban is exempt from diagnosis. GCC's acceptance is a defect.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
