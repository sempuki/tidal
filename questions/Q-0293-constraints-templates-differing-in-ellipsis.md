# Q-0293: Do constraints decide between two templates that differ only in an ellipsis?

**Status:** provisional
**Affects:** ORD-13, `tests/templates/ordering/constraints-ellipsis.cpp`, `tests/templates/ordering/constraints-ellipsis-on-second.cpp`

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

Kept open on 2026-11-03 under decision 0054, because the gap review does not support the answer with no doubt (it reads with doubt, weakly, unclearly, or for only part of the question).

Checked on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). ORD-P2 fails step 3, because step 6d of ORD-P2 sends the reader to the conditions of ORD-13. The matrix shows no model that a principle can state. ORD-5 leaves the ellipsis out of a call for templates, ORD-13 counts parameters with default arguments, which ORD-5 also leaves out, and Clang 22 alone counts the ellipsis for plain functions and not for templates. The question stays provisional.

Kept open on 2026-11-04 under decision 0055, because ORD-P2 changed in this commit: steps 1, 3, and 6 now give the type of an invented value, the pairing of packs, and the agreement of the values from all pairs. Step 6d did not change. The question waits for the next use of ORD-P2.

Used on 2026-11-04 by the principle-use batch PU2 (`reports/clean-room/PU2/`), under decision 0056. A clean-room reader applied ORD-P2, without the rule text, to the program of `constraints-ellipsis.cpp`. The answer names ORD-P1, ORD-P2, ORD-P4, and OVL-P3 and says that the call is ambiguous, which is option 2 and GCC's result. The base expectation is option 1, in which the constrained template wins. The reader marked the answer doubtful (finding PU2-2), because the model does not say how an ellipsis counts in steps 6c and 6d of ORD-P2. The use does not support the answer.

Kept open on 2026-11-04 under decisions 0054 and 0056, because the use in batch PU2 reaches option 2 with doubt (finding PU2-2). ORD-P2 does not rule out option 2. GCC 16 is the one newest family that differs.

Kept open on 2026-11-04 under decision 0055, because ORD-P2 and ORD-P4 changed in this commit: steps 6c and 6d of ORD-P2 and the text of ORD-P4 now say that an ellipsis is not a pack in 6c and does not count in 6d for two templates, and that 6d compares the declared types (findings PU2-2, R9D-2, and R9D-3). The question waits for the next use of ORD-P2.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 3 (`reports/review/2026-11-03-gap-review-question-cluster-3.md`) reads the reference as supporting option 1, with a small doubt. For two templates, the constraint step checks the template parameter lists and the function parameter lists, and an ellipsis is not a parameter, so the constraint decides. For two plain functions, the list of parameter types records the ellipsis, which fits Q-0146. The two conditions differ, as in Clang 22. The doubt is that the reference does not define a function parameter list as a term.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
