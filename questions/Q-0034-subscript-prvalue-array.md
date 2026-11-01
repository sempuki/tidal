# Q-0034: Is a subscript of an array xvalue or an array member of a prvalue an xvalue?

**Status:** provisional
**Affects:** PFX-3, VC-3, `tests/expressions/postfix/subscript-prvalue-array.cpp`, `tests/expressions/value-categories/xvalue-array-subscript.cpp`

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

## Provisional answer

Yes, an xvalue (PFX-3); EDG gives an lvalue (option 1). Clang and GCC agree, and option 1 matches member access on
an xvalue.

## Opinions
