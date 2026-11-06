# Q-0293: Do constraints decide between two templates that differ only in an ellipsis?

**Status:** provisional
**Affects:** ORD-13, `tests/templates/ordering/constraints-ellipsis.cpp`, `tests/templates/ordering/constraints-ellipsis-on-second.cpp`
**Waits for:** a review (doubt)

## Context

Two function templates have the head `template <class T>`. One takes
`T` and has a constraint. The other takes `T` and an ellipsis. A call
passes one `int`. Without the constraint, every compiler finds the call
ambiguous, so ordering by types ties.

| Compiler | Result |
|---|---|
| Clang 18, Clang 22 | the constrained template |
| GCC 14, GCC 16 | ambiguous |
| EDG 7.0 | the constrained template |

For two plain functions `f(int)` and `f(int, ...)`, Clang 22 and GCC
find the call ambiguous, and Clang 18 and EDG choose the constrained
function (Q-0146). Clang 22 is the only compiler that counts the
ellipsis for plain functions and not for templates.

## Evidence

`tests/templates/ordering/constraints-ellipsis.cpp`, with an override
for GCC. `constraints-ellipsis-on-second.cpp` puts the constraint on the
template with the ellipsis, and the compilers split the same way. With
the constraint removed, the call is ambiguous on every compiler
(`ellipsis-not-trailing-pack.cpp`).

## Options

1. For templates, the ellipsis takes no part in the conditions of
   ORD-13, and the constraint decides (Clang, EDG).
2. The ellipsis counts for templates as for plain functions, and the
   call is ambiguous (GCC). ORD-13 then states one condition for both.

## Provisional answer

Yes, the ellipsis does not count for templates (ORD-13); Clang and EDG agree, GCC differs (option 1). Two of three implementations agree. ORD-13 states the
ellipsis condition for plain functions only. Option 2 gives a simpler
model, and a contributor can argue for it with the plain-function
results of Q-0146.

The question stays open under decision 0051 (2026-11-03). Clang 22 and EDG 7.0 choose the constrained template, and GCC 16 finds the call ambiguous, so two families disagree. No principle on Tidal's pages separates the options. ORD-5 leaves the ellipsis out of the types that take part in a call, which fits option 1. ORD-13 counts the ellipsis for two functions that are not templates (Q-0146), and option 2 gives one condition for both.

Kept on 2026-11-03. The source-reader gap review of question cluster 3 agrees with option 1, with a small doubt.

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). ORD-P2 fails step 3, because step 6d of ORD-P2 sends the reader to the conditions of ORD-13. The matrix shows no model that a principle can state. ORD-5 leaves the ellipsis out of a call for templates, ORD-13 counts parameters with default arguments, which ORD-5 also leaves out, and Clang 22 alone counts the ellipsis for plain functions and not for templates. The question stays provisional.

Used on 2026-11-04 by the principle-use batch PU2 (`reports/clean-room/PU2/`), under decision 0056. A clean-room reader applied ORD-P2, without the rule text, to the program of `constraints-ellipsis.cpp`. The answer names ORD-P1, ORD-P2, ORD-P4, and OVL-P3 and says that the call is ambiguous, which is option 2 and GCC's result. The base expectation is option 1, in which the constrained template wins. The reader marked the answer doubtful (finding PU2-2), because the model does not say how an ellipsis counts in steps 6c and 6d of ORD-P2. The use does not support the answer.

Kept open on 2026-11-05 under decision 0054, because the review reads the answer with a doubt (gap review of the text phase 10 wrote, item RV4).

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 3 (`reports/review/2026-11-03-gap-review-question-cluster-3.md`) reads the reference as supporting option 1, with a small doubt. For two templates, the constraint step checks the template parameter lists and the function parameter lists, and an ellipsis is not a parameter, so the constraint decides. For two plain functions, the list of parameter types records the ellipsis, which fits Q-0146. The two conditions differ, as in Clang 22. The doubt is that the reference does not define a function parameter list as a term.

### Gap review, 2026-11-05 (expressions and templates)

The source-reader gap review of the text phase 9 wrote on expressions and templates (`reports/review/2026-11-05-gap-review-phase-9-expressions-templates.md`) used ORD-P2, ORD-P4 on ``constraints-ellipsis.cpp``, without the rule text. The model's answer: The ellipsis is no pack and does not count, the lists tie, and the constraint picks the constrained template. Matches the provisional answer: yes. Reference verdict: supports with a doubt. used ORD-P2, ORD-P4 on ``constraints-ellipsis-on-second.cpp``, without the rule text. The model's answer: The same steps, with the constraint on the template that has the ellipsis. Matches the provisional answer: yes. Reference verdict: supports with a doubt.

### Gap review, 2026-11-05 (phase 10 text, expressions and templates)

The source-reader gap review of the text phase 10 wrote on expressions and templates (`reports/review/2026-11-05-gap-review-phase-10-expressions-templates.md`) reads the question: supports with a doubt. After both deductions succeed, the reference compares the lengths of the template parameter lists and the function parameter lists, and an ellipsis is not a parameter. So the constraint decides. The doubt is that the reference does not define the function parameter list as a term.
