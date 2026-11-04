# Q-0034: Is a subscript of an array xvalue or an array member of a prvalue an xvalue?

**Status:** settled
**Affects:** PFX-3, VC-3, `tests/expressions/postfix/subscript-prvalue-array.cpp`, `tests/foundations/values/xvalue-array-subscript.cpp`

## Context

A subscript can apply to an array rvalue in these forms:

- `make().v[0]`, where `make()` returns a class with an array member
  `v`, so the array belongs to a temporary object,
- a cast to an rvalue reference to an array, then a subscript,
- an array prvalue, then a subscript.

Clang 22 and 18 and GCC 16 and 14 give each subscript as an xvalue, for
example `decltype((make().v[0]))` is `int&&`. EDG 7.0 gives an lvalue,
`int&`.

PFX-3 states the rule. VC-3 lists the subscript among the xvalues and
refers to PFX-3. Q-0099 asked the same question for VC-3 and is a
duplicate of this question.

## Evidence

The two tests in **Affects**. Each has an override for EDG.

## Options

1. An xvalue, matching a member of an xvalue or a prvalue (VC-3). EDG's
   result is a possible defect.
2. An lvalue.

## Answer

Yes, an xvalue (PFX-3); EDG gives an lvalue (option 1). Clang and GCC agree, and option 1 matches member access on
an xvalue.

Settled on 2026-11-03 under decision 0054. The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) supports this answer with no doubt, and PFX-P1 gives the same answer: a subscript keeps the array's category, an lvalue for an array lvalue and an xvalue for an array rvalue. Tidal records EDG 7.0's result as a deviation, as for the compiler-defect entries Q-0002 and Q-0003, and the tests keep their overrides for EDG 7.0.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. A subscript of an array operand is an lvalue when the array is an lvalue, and an xvalue in every other case. An array prvalue is first materialized as a temporary, which gives an xvalue. EDG's lvalue is a defect.
