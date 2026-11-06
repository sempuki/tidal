# Q-0263: Is an atomic constraint whose type is never bool an error when a short circuit skips it?

**Status:** provisional
**Affects:** CNS-9, CNS-39, `tests/templates/constraints/atomic-not-bool-skipped.cpp`, `tests/templates/constraints/atomic-not-bool-nondependent-concept.cpp`
**Waits for:** a use of CNS-P2; a review (doubt)

## Context

`concept Odd = (sizeof(T) > 100) && (sizeof(T));`. The right operand
has an integer type for every `T`. For `int`, the left operand is
false, so satisfaction never substitutes into the right operand.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | rejects: atomic constraint must be of type bool |
| GCC 14, 16, host | accepts |
| EDG 7.0 | rejects: atomic constraint must have type bool |

## Evidence

The test above. Every compiler rejects the same operand when the check reaches it (`atomic-not-bool.cpp`).

With no use of the template at all, Clang and EDG reject the concept
(`atomic-not-bool-nondependent-concept.cpp`). GCC accepts the concept,
and rejects the same operand in the requires clause of a function
template at its definition (`atomic-not-bool-nondependent-clause.cpp`).
CNS-39 states the check at the definition.

Every compiler accepts a skipped atomic constraint whose type depends on the argument, such as `T::value` for a class whose `value` is an `int` (`or-short-circuit.cpp`).

## Options

1. Not valid (Clang, EDG). The type does not depend on the
   arguments, so the constraint can never be satisfied.
2. Valid until the check reaches the operand (GCC).
3. Not valid, and no diagnostic is required. Each compiler's result
   lies within the rule.

## Provisional answer

Not valid, no diagnostic required; Clang and EDG diagnose, GCC does not (option 3). A constraint that no argument can satisfy is not valid in the template, and DEP-P6 needs no diagnostic for a construct that no argument list makes valid. CNS-39 states the answer.

Changed on 2026-11-03 to option 3. The review reads option 3, every compiler's result fits it, and DEP-P6 gives the same answer: a construct that no argument list makes valid need not be diagnosed. CNS-39 is now a `no-diagnostic` rule, and the three tests cite CNS-39 alone. CNS-9 keeps the case where a check reaches the expression.

Kept open on 2026-11-03 under decision 0054, because CNS-39 changed on the same day, and a question settles only after a reader or a review has used the changed rule (decision 0055). Every compiler's result fits the rule.

Kept open on 2026-11-05 under decision 0054, because the use of CNS-P2 did not give the provisional answer, so the principle needs a repair and a new use, and the review reads the answer with a doubt (gap review of the text phase 9 wrote, item RV3).

Used on 2026-11-05 in batch PU3 (`reports/clean-room/PU3/report.md`). CNS-P2 and CNS-P3 gave answers that do not match the tests, because they do not say when an atom whose type is never `bool` is diagnosed (finding PU3-3).

Kept open on 2026-11-05 under decision 0054, because the use in batch PU3 does not support the answer.

Kept open on 2026-11-05 under decision 0055, because CNS-P2 changed in this commit after finding PU3-3: it now says that an atom whose type does not depend on a template parameter and is not `bool` makes the program not valid, with no diagnostic required, also when no check reaches it. The question waits for the next use of CNS-P2.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on templates (`reports/review/2026-11-03-gap-review-questions-templates.md`) reads against the answer in part. Satisfaction checks the type of an atomic constraint only when it reaches the constraint. A separate rule makes the program not valid, with no diagnostic required, when an atomic constraint can never be checked successfully and no check reaches it. So the program is not valid, and GCC's acceptance conforms, as do Clang's and EDG's rejections at the definition. CNS-9 and CNS-39 need a diagnostic, which the review does not support. The same holds for the requires clause of a function template.

### Gap review, 2026-11-05 (expressions and templates)

The source-reader gap review of the text phase 9 wrote on expressions and templates (`reports/review/2026-11-05-gap-review-phase-9-expressions-templates.md`) used CNS-P2 on ``atomic-not-bool-skipped.cpp``, without the rule text. The model's answer: The left operand is false, so the right one is never substituted or checked, and the program is accepted. Matches the provisional answer: partly: the outcome is one that option 3 allows, and the model does not give "not valid". Reference verdict: supports with a doubt.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
