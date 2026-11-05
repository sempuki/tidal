# Q-0319: When is an explicit object parameter initialized, relative to the other arguments?

**Status:** settled
**Affects:** SEQ-1, SEQ-2, `tests/foundations/evaluation-order/explicit-object-order-unspecified.cpp`

## Context

A member function takes its object by value, as `this Gauge self`, and
two more parameters. The call `g.read(g, next())` copies `g` into
`self`, copies `g` into the second parameter, and calls `next()`. Each
copy constructor and `next()` record their turn.

| Compiler | First event |
|---|---|
| Clang 22, 18 | the copy into `self` |
| GCC 16, at `-O0` and `-O2` | the copy into `self` |
| GCC 14 | `next()` |

The object expression `g` is evaluated before the arguments on every
compiler (SEQ-2). The question concerns the copy into `self`.

## Evidence

The test above. A first version of the test expected the copy into
`self` first, and failed on GCC 14 only.

## Options

1. The copy into `self` takes an unspecified place among the argument
   evaluations (SEQ-1), as the initialization of any parameter does.
2. The copy into `self` comes first, as part of the object expression
   (SEQ-2).

## Answer

In an unspecified place (SEQ-1); GCC 14 differs from GCC 16 and Clang (option 1). GCC 14 and GCC 16 differ, and option 1 allows both. The test
accepts every order.

Decision 0051 does not settle the question. Clang 22 and GCC 16 both make the copy into `self` first, which both options allow. Only GCC 14 separates the options, so the newest versions give no ground to settle the answer.

Reopened as provisional on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) supports this answer with no doubt. SEQ-P1 and SEQ-P2 fail step 3 of the test. The model fixes the function before the arguments and leaves the order of the arguments to the implementation. The model does not say whether the copy into an explicit object parameter belongs to the function or to the arguments, so both options fit the model. No other principle on the page gives the answer. No newest family differs. Clang 22 and GCC 16 both make the copy into `self` first, which the answer allows, and the test accepts every order.

On 2026-11-04, item PQ widened SEQ-P1 to state the case. In a member call the object expression is part of the function, and the initialization of each parameter, an explicit object parameter included, is part of the arguments. So the copy into `self` takes a place among the arguments that SEQ-P2 leaves to the implementation. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses SEQ-P1.

Used on 2026-11-04 by the source-reader gap review of the text phase 8 wrote on expressions, foundations, and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`), Part B. The review applied SEQ-P1 to the question's program, a member call whose explicit object parameter is a copy (`explicit-object-order-unspecified.cpp`), without the rule text: the copy into the explicit object parameter is part of the arguments, so its place among them is unspecified. That is the provisional answer, and the model rules out the other options.

Settled on 2026-11-04 under decision 0054. The review supports the answer with no doubt, and SEQ-P1 gives the same answer under the test of the style guide (section 3). No newest family differs. GCC 14 makes the copy at another place, which the answer allows.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. The implied object argument comes first in the list that matches arguments to parameters, so an explicit object parameter is initialized as any other parameter is. The postfix expression is sequenced before the arguments. The initializations of the parameters are indeterminately sequenced with each other, so they do not interleave, and either order is allowed.

### Gap review, 2026-11-04

The source-reader gap review of the text phase 8 wrote on expressions, foundations, and templates (`reports/review/2026-11-04-gap-review-phase-8-expressions-foundations-templates.md`) agrees with option 1. The function part is sequenced before the arguments, the parameters are initialized in an indeterminate order, and the reference counts the object argument among the arguments for an explicit object parameter.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-11-01

The source-reader gap review of question cluster 1 (`reports/review/2026-11-01-gap-review-question-cluster-1.md`) reads the reference as supporting option 1. The implied object argument counts as the first argument, the initialization of each parameter is indeterminately sequenced with the others, and only the object expression itself must come first.
