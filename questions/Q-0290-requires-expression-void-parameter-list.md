# Q-0290: Does `(void)` give a requires expression no parameters inside a template?

**Status:** settled
**Affects:** CPT-28, `tests/templates/concepts/requires-parameters-void-template.cpp`

## Context

A concept is `requires (void) { T(); }`, and the program checks it for
`int`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | `false` |
| GCC 14, 16, host | `true` |
| EDG 7.0 | `true` |

## Evidence

The test above. Outside a template, every compiler accepts
`requires (void) { 1; }` with the value `true`
(`requires-parameters-void.cpp`). The gap review of concepts and
constraints (item 23) says that the parameter list follows the rules
for a function's parameter list.

## Options

1. `(void)` means no parameters, as in a function (GCC, EDG).
2. The requires expression is `false` inside a template (Clang).

## Answer

Yes; GCC and EDG agree, Clang differs (option 1). Two of three implementations agree, and every compiler
accepts the form outside a template.

Kept open on 2026-11-03 (replaced by the note of 2026-11-04) under decision 0054, because no principle on the page gives the answer. CPT-P3 does not describe the parameter list.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). CPT-P3 failed step 3, because CPT-P3 did not describe the parameter list. Item PQ widened CPT-P3. The list has the form of a function's parameter list inside a template as well as outside one, so `(void)` means no parameters. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses CPT-P3.

Used on 2026-11-04 by the source-reader gap review of the text phase 8 wrote on expressions, foundations, and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`), Part B. The review applied CPT-P3 to the question's program, a requires expression whose parameter list is a lone `void` (`requires-parameters-void-template.cpp`), without the rule text: the list has the form of a function's list, so a lone `void` means no parameters, which is the provisional answer.

Kept open on 2026-11-04 under decision 0054, because the review agrees with a small doubt. Two newest families also differ from the answer: GCC 16 and EDG 7.0 give `true`.

Corrected on 2026-11-05: the note above names the wrong families. GCC 16 and EDG 7.0 give `true` and follow the answer. Clang is the one newest family that differs: it finds the requires expression false inside a template, and `requires-parameters-void-template.cpp` has an override for Clang only. The question waits only on a review that reads the answer with no doubt.

Settled on 2026-11-11 under decision 0054 (decision 0066, item DQ). The second review of the doubt-held questions (`reports/review/2026-11-11-second-review-doubt-held-questions.md`) supports the answer with no doubt, and Clang 22 alone differs. CPT-P3 gives the answer. The parameter list of a requires expression has the form of a function's parameter list, inside a template as well as outside one, so `(void)` means no parameters. The model rules out option 2. CPT-P3 did not change in phase 18, so decision 0055 does not hold it back, and the gap review of 2026-11-04 used CPT-P3 on the question's program. Tidal records Clang 22's result as a deviation, and the test keeps its override for Clang.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) agrees with option 1. The parameter list of a requires expression uses the grammar and rules of a function's parameter list, so a lone unnamed parameter of type `void` that depends on nothing means an empty list, inside a template as outside. Clang's result is a defect.

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on expressions, foundations, and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`) agrees with option 1, with a small doubt. The parameters of a requires expression use the clause of a function's parameters, and the reference sends them to the function rules, where a single unnamed parameter of non-dependent `void` type means an empty list. The doubt is that the reference does not repeat that rule for requires expressions.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Second review of the doubt-held questions, 2026-11-11

The source-reader second review of the questions held only by a doubt (`reports/review/2026-11-11-second-review-doubt-held-questions.md`) reads the question: supports, and the small doubt of the 2026-11-04 review is resolved. The parameter list of a requires expression is a function parameter clause, and the reference sends the reader to the function rules for it. Those rules give a lone unnamed parameter of a `void` type that does not depend on the parameters the meaning of an empty list.
