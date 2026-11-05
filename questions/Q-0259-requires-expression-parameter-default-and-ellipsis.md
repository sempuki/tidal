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

Reopened as provisional on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) supports this answer with no doubt. CPT-P3 fails the test, because CPT-P3 names CPT-29 in its `:exceptions:` line. The model says that the parameters are names with types and that no objects exist for them. A default argument or an ellipsis would then have no use, and the model does not say that the program is rejected for it, so option 2 also fits the model. No other principle on the page gives the answer. GCC 16 differs, and the tests keep their overrides for GCC 16.

On 2026-11-04, item PQ widened CPT-P3. Nothing calls a requires expression, so its parameter list has none of the parts that serve a call: no default argument, no placeholder type, and no closing ellipsis. CPT-29 moved from the `:exceptions:` line of CPT-P3 to the `:rules:` line. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses CPT-P3.

Used on 2026-11-04 by the source-reader gap review of the text phase 8 wrote on expressions, foundations, and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`), Part B. The review applied CPT-P3 to the question's program, a requires expression whose parameter has a default argument, or whose list ends in an ellipsis (`requires-parameter-default.cpp`, `requires-parameter-ellipsis.cpp`), without the rule text: nothing calls a requires expression, so its parameters have no default argument and no ellipsis. That is the provisional answer, and the model rules out the other options.

Settled on 2026-11-04 under decision 0054. The review supports the answer with no doubt, and CPT-P3 gives the same answer under the test of the style guide (section 3). GCC 16 still differs, and the tests keep their overrides.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) agrees with option 1. A default argument on a parameter of a requires expression, and a parameter list there that ends with an ellipsis, are both forbidden, and neither ban is exempt from diagnosis. GCC's acceptance is a defect.

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on expressions, foundations, and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`) agrees with option 1. The reference forbids a default argument and a trailing ellipsis there.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
