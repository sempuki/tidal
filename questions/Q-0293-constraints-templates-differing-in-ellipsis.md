# Q-0293: Do constraints decide between two templates that differ only in an ellipsis?

**Status:** provisional
**Affects:** ORD-13, `tests/templates/ordering/constraints-ellipsis.cpp`

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
for GCC. A probe with the constraint removed is ambiguous on every
compiler.

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

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of question cluster 3 (`reports/review/2026-11-03-gap-review-question-cluster-3.md`) reads the reference as supporting option 1, with a small doubt. For two templates, the constraint step checks the template parameter lists and the function parameter lists, and an ellipsis is not a parameter, so the constraint decides. For two plain functions, the list of parameter types records the ellipsis, which fits Q-0146. The two conditions differ, as in Clang 22. The doubt is that the reference does not define a function parameter list as a term.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
