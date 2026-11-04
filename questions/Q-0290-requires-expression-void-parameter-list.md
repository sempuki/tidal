# Q-0290: Does `(void)` give a requires expression no parameters inside a template?

**Status:** provisional
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

## Provisional answer

Yes; GCC and EDG agree, Clang differs (option 1). Two of three implementations agree, and every compiler
accepts the form outside a template.

Kept open on 2026-11-03 under decision 0054, because no principle on the page gives the answer. CPT-P3 does not describe the parameter list.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). CPT-P3 failed step 3, because CPT-P3 did not describe the parameter list. Item PQ widened CPT-P3. The list has the form of a function's parameter list inside a template as well as outside one, so `(void)` means no parameters. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses CPT-P3.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) agrees with option 1. The parameter list of a requires expression uses the grammar and rules of a function's parameter list, so a lone unnamed parameter of type `void` that depends on nothing means an empty list, inside a template as outside. Clang's result is a defect.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
