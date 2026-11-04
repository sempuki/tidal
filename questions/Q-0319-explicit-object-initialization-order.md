# Q-0319: When is an explicit object parameter initialized, relative to the other arguments?

**Status:** provisional
**Affects:** SEQ-1, SEQ-2, `tests/expressions/sequencing/explicit-object-order-unspecified.cpp`

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

## Provisional answer

In an unspecified place (SEQ-1); GCC 14 differs from GCC 16 and Clang (option 1). GCC 14 and GCC 16 differ, and option 1 allows both. The test
accepts every order.

Decision 0051 does not settle the question. Clang 22 and GCC 16 both make the copy into `self` first, which both options allow. Only GCC 14 separates the options, so the newest versions give no ground to settle the answer.

Reopened as provisional on 2026-11-04 under the principle test of the style guide (section 3, item PQ of decision 0055). The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) supports this answer with no doubt. SEQ-P1 and SEQ-P2 fail step 3 of the test. The model fixes the function before the arguments and leaves the order of the arguments to the implementation. The model does not say whether the copy into an explicit object parameter belongs to the function or to the arguments, so both options fit the model. No other principle on the page gives the answer. No newest family differs. Clang 22 and GCC 16 both make the copy into `self` first, which the answer allows, and the test accepts every order.

On 2026-11-04, item PQ widened SEQ-P1 to state the case. In a member call the object expression is part of the function, and the initialization of each parameter, an explicit object parameter included, is part of the arguments. So the copy into `self` takes a place among the arguments that SEQ-P2 leaves to the implementation. Under decision 0055, the question does not settle on a principle that phase 8 wrote. The question stays provisional until a reader or a reviewer uses SEQ-P1.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. The implied object argument comes first in the list that matches arguments to parameters, so an explicit object parameter is initialized as any other parameter is. The postfix expression is sequenced before the arguments. The initializations of the parameters are indeterminately sequenced with each other, so they do not interleave, and either order is allowed.

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->

### Gap review, 2026-11-01

The source-reader gap review of question cluster 1 (`reports/review/2026-11-01-gap-review-question-cluster-1.md`) reads the reference as supporting option 1. The implied object argument counts as the first argument, the initialization of each parameter is indeterminately sequenced with the others, and only the object expression itself must come first.
